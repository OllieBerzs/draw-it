use log::debug;
use log::error;
use log::info;
use std::collections::HashSet;
use std::process::exit;
use winit::dpi::PhysicalPosition;
use winit::dpi::PhysicalSize;
use winit::event::DeviceEvent;
use winit::event::ElementState;
use winit::event::Event;
use winit::event::KeyboardInput;
pub use winit::event::VirtualKeyCode as Key;
use winit::event::WindowEvent;
use winit::event_loop::ControlFlow;
use winit::event_loop::EventLoop;
use winit::window::Window as WinitWindow;
use winit::window::WindowBuilder;

pub struct Window {
    event_loop: EventLoop<()>,
    window: WinitWindow,
}

pub struct Events {
    mouse_position: (u32, u32),
    mouse_delta: (f32, f32),
    keys: Keys,
    window: WinitWindow,
}

#[derive(Default)]
struct Keys {
    pressed: HashSet<Key>,
    released: HashSet<Key>,
    typed: HashSet<Key>,
}

impl Window {
    pub fn new(width: u32, height: u32) -> Self {
        let event_loop = EventLoop::new();

        debug!("create window");
        let window = WindowBuilder::new()
            .with_resizable(false)
            .with_inner_size(PhysicalSize::new(width, height))
            .build(&event_loop)
            .or_error("cannot create window");
        info!("window created");

        Self { event_loop, window }
    }

    pub fn start_loop<F: FnMut(&Events) + 'static>(self, mut draw: F) {
        let event_loop = self.event_loop;
        let window = self.window;

        let mut events = Events {
            mouse_position: (0, 0),
            mouse_delta: (0.0, 0.0),
            keys: Keys::default(),
            window,
        };

        debug!("start event loop");
        event_loop.run(move |event, _, control_flow| {
            *control_flow = ControlFlow::Poll;
            match event {
                Event::WindowEvent {
                    event: win_event, ..
                } => match win_event {
                    WindowEvent::CursorMoved { position: pos, .. } => {
                        events.mouse_position = (pos.x as u32, pos.y as u32);
                    }
                    WindowEvent::KeyboardInput {
                        input:
                            KeyboardInput {
                                virtual_keycode: Some(keycode),
                                state,
                                ..
                            },
                        ..
                    } => events.keys.handle(keycode, state),
                    WindowEvent::CloseRequested => {
                        debug!("close window");
                        *control_flow = ControlFlow::Exit;
                    }
                    _ => (),
                },
                Event::DeviceEvent {
                    event: dev_event, ..
                } => match dev_event {
                    DeviceEvent::MouseMotion { delta, .. } => {
                        events.mouse_delta = (delta.0 as f32, delta.1 as f32);
                    }
                    _ => (),
                },
                Event::MainEventsCleared => {
                    draw(&events);
                    events.keys.clear_typed();
                }
                _ => (),
            }
        });
    }

    #[cfg(target_os = "windows")]
    pub fn hwnd(&self) -> *mut std::ffi::c_void {
        use winit::platform::windows::WindowExtWindows;
        self.window.hwnd()
    }

    #[cfg(target_os = "linux")]
    pub fn xlib_window(&self) -> std::os::raw::c_ulong {
        use winit::platform::unix::WindowExtUnix;
        self.window.xlib_window().or_error("no xlib support")
    }

    #[cfg(target_os = "linux")]
    pub fn xlib_display(&self) -> *mut std::ffi::c_void {
        use winit::platform::unix::WindowExtUnix;
        self.window.xlib_display().or_error("no xlib support")
    }

    #[cfg(target_os = "macos")]
    pub fn ns_window(&self) -> *mut std::ffi::c_void {
        use winit::platform::macos::WindowExtMacOS;
        self.window.ns_window()
    }

    #[cfg(target_os = "macos")]
    pub fn ns_view(&self) -> *mut std::ffi::c_void {
        use winit::platform::macos::WindowExtMacOS;
        self.window.ns_view()
    }

    pub fn size(&self) -> (u32, u32) {
        let size = self.window.inner_size();
        (size.width, size.height)
    }
}

impl Events {
    pub fn size(&self) -> (u32, u32) {
        let size = self.window.inner_size();
        (size.width, size.height)
    }

    pub fn mouse_position(&self) -> (u32, u32) {
        self.mouse_position
    }

    pub fn mouse_delta(&self) -> (f32, f32) {
        self.mouse_delta
    }

    pub fn set_title(&self, title: impl AsRef<str>) {
        self.window.set_title(title.as_ref());
    }

    pub fn set_size(&self, width: u32, height: u32) {
        self.window.set_inner_size(PhysicalSize::new(width, height));
    }

    pub fn set_mouse_position(&self, x: u32, y: u32) {
        self.window
            .set_cursor_position(PhysicalPosition::new(x, y))
            .or_error("cannot change mouse position on iOS");
    }

    pub fn set_cursor_grab(&self, grab: bool) {
        self.window
            .set_cursor_grab(grab)
            .or_error("cannot grab mouse on iOS");
    }

    pub fn set_cursor_visible(&self, visible: bool) {
        self.window.set_cursor_visible(visible);
    }

    pub fn set_visible(&self, visible: bool) {
        self.window.set_visible(visible);
    }

    pub fn is_key_pressed(&self, key: Key) -> bool {
        self.keys.is_pressed(key)
    }

    pub fn is_key_released(&self, key: Key) -> bool {
        self.keys.is_released(key)
    }

    pub fn is_key_typed(&self, key: Key) -> bool {
        self.keys.is_typed(key)
    }
}

impl Keys {
    pub(crate) fn handle(&mut self, key: Key, state: ElementState) {
        match state {
            ElementState::Pressed => {
                self.pressed.insert(key);
                self.typed.insert(key);
                self.released.remove(&key);
            }
            ElementState::Released => {
                self.released.insert(key);
                self.pressed.remove(&key);
                self.typed.remove(&key);
            }
        }
    }

    pub(crate) fn clear_typed(&mut self) {
        self.typed.clear();
    }

    pub(crate) fn is_pressed(&self, key: Key) -> bool {
        self.pressed.contains(&key)
    }

    pub(crate) fn is_released(&self, key: Key) -> bool {
        self.released.contains(&key)
    }

    pub(crate) fn is_typed(&self, key: Key) -> bool {
        self.typed.contains(&key)
    }
}

trait OrError<T> {
    fn or_error(self, msg: impl AsRef<str>) -> T;
}

impl<T, E> OrError<T> for Result<T, E> {
    fn or_error(self, msg: impl AsRef<str>) -> T {
        self.unwrap_or_else(|_| {
            error!("{}", msg.as_ref());
            exit(1);
        })
    }
}

impl<T> OrError<T> for Option<T> {
    fn or_error(self, msg: impl AsRef<str>) -> T {
        self.unwrap_or_else(|| {
            error!("{}", msg.as_ref());
            exit(1);
        })
    }
}
