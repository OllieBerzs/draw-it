// Oliver Berzs
// https://github.com/OllieBerzs/tegne-rs

// UI struct with framebuffer that displays framerate

use imgui::im_str;
use imgui::Condition;
use imgui::Context;
use imgui::FontConfig;
use imgui::FontSource;
use imgui::Window as ImWindow;
use tegne::Camera;
use tegne::Events;
use tegne::Framebuffer;
use tegne::Id;
use tegne::Mesh;
use tegne::MeshOptions;
use tegne::Shader;
use tegne::ShaderOptions;
use tegne::Tegne;
use tegne::Texture;
use tegne::Transform;
use tegne::Vector2;
use tegne::Vector3;
use tegne::Vector4;

pub struct Ui {
    framebuffer: Id<Framebuffer>,
    mesh: Id<Mesh>,
    texture: Id<Texture>,
    shader: Id<Shader>,
    camera: Camera,
    imgui: Context,
    height: u32,
}

impl Ui {
    pub fn new(tegne: &Tegne, width: u32, height: u32) -> Self {
        let framebuffer = tegne.create_framebuffer(width, height);
        let camera = Camera::orthographic(width, height);

        let mut imgui = Context::create();
        imgui.set_ini_filename(None);
        imgui.io_mut().display_size = [width as f32, height as f32];

        let texture = {
            let mut fonts = imgui.fonts();
            fonts.add_font(&[FontSource::DefaultFontData {
                config: Some(FontConfig {
                    size_pixels: 13.0,
                    ..Default::default()
                }),
            }]);
            let font_tex = fonts.build_rgba32_texture();
            tegne.create_texture_rgba(font_tex.data, font_tex.width, font_tex.height)
        };

        let mesh = tegne.create_mesh(MeshOptions {
            vertices: &[Vector3::new(0.0, 0.0, 0.0)],
            triangles: &[[0, 0, 0]],
            ..Default::default()
        });

        let shader = tegne
            .create_shader_from_file(
                "examples/blue/src/test.shader",
                ShaderOptions {
                    depth_test: false,
                    ..Default::default()
                },
            )
            .unwrap();

        Self {
            framebuffer,
            mesh,
            texture,
            camera,
            imgui,
            shader,
            height,
        }
    }

    pub fn resize(&mut self, width: u32, height: u32) {
        self.camera.resize(width, height);
        self.imgui.io_mut().display_size = [width as f32, height as f32];
    }

    pub fn draw_ui(&mut self, tegne: &Tegne, events: &Events) {
        let ui = self.imgui.frame();

        ImWindow::new(im_str!("Hello world!"))
            .size([200.0, 200.0], Condition::FirstUseEver)
            .build(&ui, || {
                let fps = format!("{} fps", events.fps());
                ui.text(fps);
            });

        let draw_data = ui.render();
        let mut triangles = vec![];
        let mut vertices = vec![];
        let mut normals = vec![];
        let mut colors = vec![];
        let mut uvs = vec![];
        for draw_list in draw_data.draw_lists() {
            for tri in draw_list.idx_buffer().chunks(3) {
                triangles.push([tri[0] as u32, tri[1] as u32, tri[2] as u32]);
            }
            for vert in draw_list.vtx_buffer() {
                let vertex =
                    Vector3::new(vert.pos[0], self.height as f32 - 200.0 - vert.pos[1], 1.0);
                let uv = Vector2::new(vert.uv[0], vert.uv[1]);
                let color = Vector4::new(
                    vert.col[0] as f32 / 255.0,
                    vert.col[1] as f32 / 255.0,
                    vert.col[2] as f32 / 255.0,
                    vert.col[3] as f32 / 255.0,
                );
                vertices.push(vertex);
                uvs.push(uv);
                colors.push(color);
                normals.push(Vector3::backward());
            }
        }

        tegne.with_mesh(&self.mesh, |mesh| {
            mesh.set_vertices(&vertices);
            mesh.set_normals(&normals);
            mesh.set_colors(&colors);
            mesh.set_uvs(&uvs);
            mesh.set_triangles(&triangles);
        });

        tegne.draw(&self.framebuffer, &self.camera, |target| {
            target.set_clear_color([0.0, 0.0, 0.0, 0.0]);
            target.set_shader(&self.shader);
            target.set_albedo_texture(&self.texture);
            target.draw(&self.mesh, Transform::default());
        });
    }

    pub fn framebuffer(&self) -> &Id<Framebuffer> {
        &self.framebuffer
    }
}