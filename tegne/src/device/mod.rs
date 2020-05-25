// Oliver Berzs
// https://github.com/OllieBerzs/tegne-rs

// Device - struct to access GPU API layer

mod commands;
mod extension;
mod pick;
mod properties;

use ash::extensions::khr::Swapchain as SwapchainExt;
use ash::version::DeviceV1_0;
use ash::vk;
use ash::Device as VkDevice;
use std::ffi::c_void;
use std::mem;
use std::slice;
use std::sync::atomic::AtomicUsize;
use std::sync::atomic::Ordering;

pub(crate) use commands::Commands;
pub(crate) use commands::LayoutChangeOptions;
pub(crate) use pick::pick_gpu;
pub(crate) use properties::DeviceProperties;

use crate::buffer::BufferAccess;
use crate::error::ErrorKind;
use crate::error::ErrorType;
use crate::error::Result;
use crate::image::Framebuffer;
use crate::image::ImageLayout;
use crate::image::ImageMemory;
use crate::image::ImageSamples;
use crate::instance::layer;
use crate::instance::Instance;
use crate::pipeline::Descriptor;
use crate::pipeline::PushConstants;
use crate::pipeline::RenderPass;
use crate::pipeline::Shader;
use crate::pipeline::ShaderLayout;
use crate::sync::fence;
use crate::sync::semaphore;
use crate::window::SurfaceProperties;
use crate::window::Swapchain;

pub(crate) const IN_FLIGHT_FRAME_COUNT: usize = 2;

pub(crate) struct Device {
    handle: VkDevice,
    device_properties: DeviceProperties,
    swapchain_ext: SwapchainExt,
    graphics_queue: (u32, vk::Queue),
    present_queue: (u32, vk::Queue),
    sync_acquire_image: Vec<vk::Semaphore>,
    sync_release_image: Vec<vk::Semaphore>,
    sync_queue_submit: Vec<vk::Fence>,
    current_frame: AtomicUsize,
}

impl Device {
    pub(crate) fn new(
        instance: &Instance,
        surface_properties: &SurfaceProperties,
        device_properties: DeviceProperties,
        gpu_index: usize,
    ) -> Result<Self> {
        // configure device features
        let features = vk::PhysicalDeviceFeatures::builder()
            .sampler_anisotropy(true)
            .fill_mode_non_solid(true)
            .wide_lines(true);

        // configure queues
        let g_index = surface_properties.graphics_index();
        let p_index = surface_properties.present_index();
        let queue_priorities = [1.0];

        let g_queue_info = vk::DeviceQueueCreateInfo::builder()
            .queue_family_index(g_index)
            .queue_priorities(&queue_priorities)
            .build();
        let p_queue_info = vk::DeviceQueueCreateInfo::builder()
            .queue_family_index(p_index)
            .queue_priorities(&queue_priorities)
            .build();

        let mut queue_infos = vec![g_queue_info];
        if g_index != p_index {
            queue_infos.push(p_queue_info);
        }

        let extension_list = extension::list()?;
        let extensions = extension::to_i8(&extension_list);
        let layer_list = layer::list()?;
        let layers = layer::to_i8(&layer_list);

        // open GPU
        let info = vk::DeviceCreateInfo::builder()
            .queue_create_infos(&queue_infos)
            .enabled_features(&features)
            .enabled_layer_names(&layers)
            .enabled_extension_names(&extensions);

        let handle = instance.create_device(gpu_index, &info)?;

        // create swapchain extension
        let swapchain_ext = instance.create_swapchain_extension(&handle);

        // get device queues
        let graphics_queue = unsafe { handle.get_device_queue(g_index, 0) };
        let present_queue = unsafe { handle.get_device_queue(p_index, 0) };

        // create synchronization objects
        let mut sync_acquire_image = vec![];
        let mut sync_release_image = vec![];
        let mut sync_queue_submit = vec![];
        for _ in 0..IN_FLIGHT_FRAME_COUNT {
            sync_acquire_image.push(semaphore::create(&handle)?);
            sync_release_image.push(semaphore::create(&handle)?);
            sync_queue_submit.push(fence::create(&handle)?);
        }

        Ok(Self {
            handle,
            device_properties,
            swapchain_ext,
            graphics_queue: (g_index, graphics_queue),
            present_queue: (p_index, present_queue),
            sync_acquire_image,
            sync_release_image,
            sync_queue_submit,
            current_frame: AtomicUsize::new(0),
        })
    }

    pub(crate) fn current_frame(&self) -> usize {
        self.current_frame.load(Ordering::Relaxed)
    }

    pub(crate) fn next_frame(&self, swapchain: &Swapchain) -> Result<()> {
        let mut current = self.current_frame();
        current = (current + 1) % IN_FLIGHT_FRAME_COUNT;

        swapchain.next(self.sync_acquire_image[current])?;

        // wait for queue
        let wait = self.sync_queue_submit[current];
        fence::wait_for(&self.handle, wait)?;
        fence::reset(&self.handle, wait)?;

        self.current_frame.store(current, Ordering::Release);

        Ok(())
    }

    pub(crate) fn submit_and_wait(&self, buffer: vk::CommandBuffer) -> Result<()> {
        let buffers = [buffer];
        let info = vk::SubmitInfo::builder().command_buffers(&buffers).build();
        let infos = [info];

        unsafe {
            self.handle
                .queue_submit(self.graphics_queue.1, &infos, vk::Fence::null())?;
            self.handle.device_wait_idle()?;
        }
        Ok(())
    }

    pub(crate) fn submit(&self, buffer: vk::CommandBuffer) -> Result<()> {
        let current = self.current_frame();
        let wait = [self.sync_acquire_image[current]];
        let signal = [self.sync_release_image[current]];
        let done = self.sync_queue_submit[current];
        let buffers = [buffer];
        let stage_mask = [vk::PipelineStageFlags::COLOR_ATTACHMENT_OUTPUT];

        let info = [vk::SubmitInfo::builder()
            .wait_semaphores(&wait)
            .signal_semaphores(&signal)
            .wait_dst_stage_mask(&stage_mask)
            .command_buffers(&buffers)
            .build()];
        unsafe {
            self.handle
                .queue_submit(self.graphics_queue.1, &info, done)?
        };
        Ok(())
    }

    pub(crate) fn present(&self, swapchain: &Swapchain) -> Result<()> {
        let current = self.current_frame();
        let wait = self.sync_release_image[current];
        swapchain.present(wait)?;
        Ok(())
    }

    pub(crate) fn wait_for_idle(&self) -> Result<()> {
        for fen in self.sync_queue_submit.iter() {
            fence::wait_for(&self.handle, *fen)?;
        }

        unsafe {
            self.handle.queue_wait_idle(self.graphics_queue.1)?;
            self.handle.queue_wait_idle(self.present_queue.1)?;
            self.handle.device_wait_idle()?;
        }
        Ok(())
    }

    pub(crate) fn create_swapchain(
        &self,
        info: &vk::SwapchainCreateInfoKHR,
    ) -> Result<vk::SwapchainKHR> {
        Ok(unsafe { self.swapchain_ext.create_swapchain(info, None)? })
    }

    pub(crate) fn destroy_swapchain(&self, handle: vk::SwapchainKHR) {
        unsafe {
            self.swapchain_ext.destroy_swapchain(handle, None);
        }
    }

    pub(crate) fn get_swapchain_images(&self, handle: vk::SwapchainKHR) -> Result<Vec<vk::Image>> {
        Ok(unsafe { self.swapchain_ext.get_swapchain_images(handle)? })
    }

    pub(crate) fn get_next_swapchain_image(
        &self,
        handle: vk::SwapchainKHR,
        signal: vk::Semaphore,
    ) -> Result<u32> {
        Ok(unsafe {
            self.swapchain_ext
                .acquire_next_image(handle, u64::max_value(), signal, Default::default())?
                .0
        })
    }

    pub(crate) fn present_queue(&self, info: &vk::PresentInfoKHR) -> Result<()> {
        unsafe {
            self.swapchain_ext
                .queue_present(self.present_queue.1, info)?;
        }
        Ok(())
    }

    pub(crate) fn graphics_index(&self) -> u32 {
        self.graphics_queue.0
    }

    pub(crate) fn find_memory_type(
        &self,
        type_filter: u32,
        props: vk::MemoryPropertyFlags,
    ) -> Option<u32> {
        self.device_properties
            .memory
            .memory_types
            .iter()
            .enumerate()
            .find(|(i, mem_type)| {
                let byte: u32 = 1 << i;
                (type_filter & byte != 0) && (mem_type.property_flags & props) == props
            })
            .map(|t| t.0 as u32)
    }

    pub(crate) fn samples(&self) -> ImageSamples {
        self.device_properties.samples
    }

    pub(crate) fn is_msaa(&self) -> bool {
        self.samples() != ImageSamples(1)
    }

    pub(crate) fn allocate_buffer(
        &self,
        info: &vk::BufferCreateInfo,
        access: BufferAccess,
    ) -> Result<(vk::Buffer, vk::DeviceMemory)> {
        // create buffer handle
        let buffer = unsafe { self.handle.create_buffer(info, None)? };

        // allocate memory
        let requirements = unsafe { self.handle.get_buffer_memory_requirements(buffer) };
        let mem_type = self
            .find_memory_type(requirements.memory_type_bits, access.flag())
            .ok_or(ErrorType::Internal(ErrorKind::UnsupportedMemoryType))?;
        let alloc_info = vk::MemoryAllocateInfo::builder()
            .allocation_size(requirements.size)
            .memory_type_index(mem_type);
        let memory = unsafe { self.handle.allocate_memory(&alloc_info, None)? };

        // bind memory
        unsafe { self.handle.bind_buffer_memory(buffer, memory, 0)? };

        Ok((buffer, memory))
    }

    pub(crate) fn free_buffer(&self, handle: vk::Buffer, memory: vk::DeviceMemory) {
        unsafe {
            self.handle.destroy_buffer(handle, None);
            self.handle.free_memory(memory, None);
        }
    }

    pub(crate) fn map_memory(
        &self,
        memory: vk::DeviceMemory,
        size: usize,
        fun: impl Fn(*mut c_void),
    ) -> Result<()> {
        let mem = unsafe {
            self.handle
                .map_memory(memory, 0, (size as u32).into(), vk::MemoryMapFlags::empty())?
        };
        fun(mem);
        unsafe {
            self.handle.unmap_memory(memory);
        }
        Ok(())
    }

    pub(crate) fn create_command_pool(&self) -> Result<vk::CommandPool> {
        let info = vk::CommandPoolCreateInfo::builder()
            .flags(vk::CommandPoolCreateFlags::TRANSIENT)
            .queue_family_index(self.graphics_index());
        Ok(unsafe { self.handle.create_command_pool(&info, None)? })
    }

    pub(crate) fn destroy_command_pool(&self, pool: vk::CommandPool) {
        unsafe {
            self.handle.destroy_command_pool(pool, None);
        }
    }

    pub(crate) fn allocate_command_buffer(
        &self,
        info: &vk::CommandBufferAllocateInfo,
    ) -> Result<vk::CommandBuffer> {
        Ok(unsafe { self.handle.allocate_command_buffers(&info)?[0] })
    }

    pub(crate) fn free_command_buffer(
        &self,
        pool: vk::CommandPool,
        buffer: vk::CommandBuffer,
    ) -> Result<()> {
        let buffers = [buffer];
        unsafe {
            self.handle
                .reset_command_pool(pool, vk::CommandPoolResetFlags::RELEASE_RESOURCES)?;
            self.handle.free_command_buffers(pool, &buffers);
        }
        Ok(())
    }

    pub(crate) fn begin_command_buffer(&self, buffer: vk::CommandBuffer) -> Result<()> {
        let info = vk::CommandBufferBeginInfo::builder()
            .flags(vk::CommandBufferUsageFlags::ONE_TIME_SUBMIT);
        unsafe {
            self.handle.begin_command_buffer(buffer, &info)?;
        }
        Ok(())
    }

    pub(crate) fn end_command_buffer(&self, buffer: vk::CommandBuffer) -> Result<()> {
        unsafe {
            self.handle.end_command_buffer(buffer)?;
        }
        Ok(())
    }

    pub(crate) fn cmd_begin_render_pass(
        &self,
        buffer: vk::CommandBuffer,
        framebuffer: &Framebuffer,
        render_pass: &RenderPass,
        clear: [f32; 4],
    ) {
        // create clear values based on framebuffer image formats
        let clear_values = framebuffer
            .iter_images()
            .map(|image| {
                if image.has_depth_format() {
                    vk::ClearValue {
                        depth_stencil: vk::ClearDepthStencilValue {
                            depth: 1.0,
                            stencil: 0,
                        },
                    }
                } else {
                    vk::ClearValue {
                        color: vk::ClearColorValue { float32: clear },
                    }
                }
            })
            .collect::<Vec<_>>();

        let info = vk::RenderPassBeginInfo::builder()
            .render_pass(render_pass.handle())
            .framebuffer(framebuffer.handle())
            .render_area(vk::Rect2D {
                offset: vk::Offset2D { x: 0, y: 0 },
                extent: vk::Extent2D {
                    width: framebuffer.width(),
                    height: framebuffer.height(),
                },
            })
            .clear_values(&clear_values);
        unsafe {
            self.handle
                .cmd_begin_render_pass(buffer, &info, vk::SubpassContents::INLINE);
        }
    }

    pub(crate) fn cmd_end_render_pass(&self, buffer: vk::CommandBuffer) {
        unsafe {
            self.handle.cmd_end_render_pass(buffer);
        }
    }

    pub(crate) fn cmd_bind_shader(&self, buffer: vk::CommandBuffer, shader: &Shader) {
        unsafe {
            self.handle
                .cmd_bind_pipeline(buffer, vk::PipelineBindPoint::GRAPHICS, shader.handle());
        }
    }

    pub(crate) fn cmd_bind_descriptor(
        &self,
        buffer: vk::CommandBuffer,
        descriptor: Descriptor,
        layout: &ShaderLayout,
    ) {
        let sets = [descriptor.1];
        unsafe {
            self.handle.cmd_bind_descriptor_sets(
                buffer,
                vk::PipelineBindPoint::GRAPHICS,
                layout.handle(),
                descriptor.0,
                &sets,
                &[],
            );
        }
    }

    pub(crate) fn cmd_bind_vertex_buffer(&self, buffer: vk::CommandBuffer, v_buffer: vk::Buffer) {
        let buffers = [v_buffer];
        let offsets = [0];
        unsafe {
            self.handle
                .cmd_bind_vertex_buffers(buffer, 0, &buffers, &offsets);
        }
    }

    pub(crate) fn cmd_bind_index_buffer(&self, buffer: vk::CommandBuffer, i_buffer: vk::Buffer) {
        unsafe {
            self.handle
                .cmd_bind_index_buffer(buffer, i_buffer, 0, vk::IndexType::UINT32);
        }
    }

    pub(crate) fn cmd_push_constants(
        &self,
        buffer: vk::CommandBuffer,
        constants: PushConstants,
        layout: &ShaderLayout,
    ) {
        unsafe {
            let data: &[u8] = slice::from_raw_parts(
                &constants as *const PushConstants as *const u8,
                mem::size_of::<PushConstants>(),
            );

            self.handle.cmd_push_constants(
                buffer,
                layout.handle(),
                vk::ShaderStageFlags::VERTEX | vk::ShaderStageFlags::FRAGMENT,
                0,
                data,
            );
        }
    }

    pub(crate) fn cmd_draw(&self, buffer: vk::CommandBuffer, count: u32) {
        unsafe {
            self.handle.cmd_draw_indexed(buffer, count, 1, 0, 0, 0);
        }
    }

    pub(crate) fn cmd_copy_buffer(
        &self,
        buffer: vk::CommandBuffer,
        src: vk::Buffer,
        dst: vk::Buffer,
        size: usize,
    ) {
        let region = [vk::BufferCopy::builder()
            .src_offset(0)
            .dst_offset(0)
            .size((size as u32).into())
            .build()];
        unsafe {
            self.handle.cmd_copy_buffer(buffer, src, dst, &region);
        }
    }

    pub(crate) fn cmd_copy_buffer_to_image(
        &self,
        buffer: vk::CommandBuffer,
        src: vk::Buffer,
        dst: vk::Image,
        region: vk::BufferImageCopy,
    ) {
        let regions = [region];
        unsafe {
            self.handle.cmd_copy_buffer_to_image(
                buffer,
                src,
                dst,
                ImageLayout::TransferDst.flag(),
                &regions,
            );
        }
    }

    pub(crate) fn cmd_set_view(&self, buffer: vk::CommandBuffer, width: u32, height: u32) {
        let viewport = [vk::Viewport {
            x: 0.0,
            y: 0.0,
            width: width as f32,
            height: height as f32,
            min_depth: 0.0,
            max_depth: 1.0,
        }];
        let scissor = [vk::Rect2D {
            offset: vk::Offset2D { x: 0, y: 0 },
            extent: vk::Extent2D { width, height },
        }];

        unsafe {
            self.handle.cmd_set_viewport(buffer, 0, &viewport);
            self.handle.cmd_set_scissor(buffer, 0, &scissor);
        }
    }

    pub(crate) fn cmd_set_line_width(&self, buffer: vk::CommandBuffer, width: f32) {
        unsafe {
            self.handle.cmd_set_line_width(buffer, width);
        }
    }

    pub(crate) fn cmd_blit_image(
        &self,
        buffer: vk::CommandBuffer,
        src: vk::Image,
        dst: vk::Image,
        blit: vk::ImageBlit,
        filter: vk::Filter,
    ) {
        let regions = [blit];
        unsafe {
            self.handle.cmd_blit_image(
                buffer,
                src,
                ImageLayout::TransferSrc.flag(),
                dst,
                ImageLayout::TransferDst.flag(),
                &regions,
                filter,
            );
        }
    }

    pub(crate) fn cmd_change_image_layout(
        &self,
        buffer: vk::CommandBuffer,
        image: &ImageMemory,
        options: LayoutChangeOptions,
    ) {
        let src_access = options.old_layout.access_flag();
        let dst_access = options.new_layout.access_flag();
        let src_stage = options.old_layout.stage_flag();
        let dst_stage = options.new_layout.stage_flag();
        let aspect_mask = if image.has_depth_format() {
            vk::ImageAspectFlags::DEPTH | vk::ImageAspectFlags::STENCIL
        } else {
            vk::ImageAspectFlags::COLOR
        };

        let subresource = vk::ImageSubresourceRange::builder()
            .aspect_mask(aspect_mask)
            .base_array_layer(0)
            .base_mip_level(options.base_mip)
            .layer_count(1)
            .level_count(options.mip_count)
            .build();
        let barrier = [vk::ImageMemoryBarrier::builder()
            .src_queue_family_index(vk::QUEUE_FAMILY_IGNORED)
            .dst_queue_family_index(vk::QUEUE_FAMILY_IGNORED)
            .subresource_range(subresource)
            .image(image.handle())
            .old_layout(options.old_layout.flag())
            .new_layout(options.new_layout.flag())
            .src_access_mask(src_access)
            .dst_access_mask(dst_access)
            .build()];

        unsafe {
            self.handle.cmd_pipeline_barrier(
                buffer,
                src_stage,
                dst_stage,
                vk::DependencyFlags::default(),
                &[],
                &[],
                &barrier,
            );
        }
    }

    pub(crate) fn logical(&self) -> &VkDevice {
        &self.handle
    }
}

impl Drop for Device {
    fn drop(&mut self) {
        unsafe {
            self.sync_acquire_image
                .iter()
                .for_each(|s| semaphore::destroy(&self.handle, *s));
            self.sync_release_image
                .iter()
                .for_each(|s| semaphore::destroy(&self.handle, *s));
            self.sync_queue_submit
                .iter()
                .for_each(|f| fence::destroy(&self.handle, *f));
            self.handle.destroy_device(None);
        }
    }
}
