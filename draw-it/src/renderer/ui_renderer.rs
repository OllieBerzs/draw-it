#![cfg(feature = "ui")]

use imgui::Ui;
use std::sync::Arc;

use crate::camera::CameraType;
use crate::color::Color;
use crate::device::Device;
use crate::error::Result;
use crate::image::Framebuffer;
use crate::image::FramebufferOptions;
use crate::image::ImageFormat;
use crate::image::Texture;
use crate::image::TextureOptions;
use crate::math::Matrix4;
use crate::math::Vector2;
use crate::math::Vector3;
use crate::mesh::Mesh;
use crate::mesh::MeshOptions;
use crate::pipeline::CullMode;
use crate::pipeline::DepthMode;
use crate::pipeline::ImageUniform;
use crate::pipeline::PushConstants;
use crate::pipeline::Shader;
use crate::pipeline::ShaderLayout;
use crate::pipeline::ShaderOptions;
use crate::pipeline::WorldData;
use crate::resource::Ref;
use crate::resource::ResourceManager;

pub(crate) struct UiRenderer {
    framebuffer: Ref<Framebuffer>,
    shader: Shader,
    mesh: Mesh,
    texture: Option<Texture>,
    drawn: bool,
    device: Arc<Device>,
}

impl UiRenderer {
    pub(crate) fn new(
        device: &Arc<Device>,
        shader_layout: &ShaderLayout,
        image_uniform: &ImageUniform,
        resources: &mut ResourceManager,
        width: u32,
        height: u32,
    ) -> Result<Self> {
        profile_scope!("new");

        let framebuffer = Framebuffer::new(
            device,
            shader_layout,
            image_uniform,
            FramebufferOptions {
                attachment_formats: &[ImageFormat::Sbgra],
                camera_type: CameraType::Orthographic,
                multisampled: false,
                depth: false,
                width,
                height,
            },
        )?;

        let shader = Shader::new(
            device,
            &framebuffer,
            shader_layout,
            include_bytes!("../../shaders/ui.shader"),
            ShaderOptions {
                depth_mode: DepthMode::Disabled,
                cull_mode: CullMode::Disabled,
                ..Default::default()
            },
        )?;

        let mesh = Mesh::new(
            device,
            MeshOptions {
                vertices: &[Vector3::new(0.0, 0.0, 0.0)],
                indices: &[0, 0, 0],
                ..Default::default()
            },
        )?;

        Ok(Self {
            device: Arc::clone(device),
            framebuffer: resources.add_framebuffer(framebuffer),
            texture: None,
            drawn: false,
            shader,
            mesh,
        })
    }

    pub(crate) fn draw(&mut self, ui: Ui<'_>, shader_layout: &ShaderLayout) -> Result<()> {
        let draw_data = ui.render();

        let half_width = draw_data.display_size[0] / 2.0;
        let half_height = draw_data.display_size[1] / 2.0;

        // generate mesh data
        let mut indices = vec![];
        let mut vertices = vec![];
        let mut normals = vec![];
        let mut colors = vec![];
        let mut uvs = vec![];
        let mut to = 0;
        for draw_list in draw_data.draw_lists() {
            indices.extend(draw_list.idx_buffer().iter().map(|i| *i as u32 + to));
            for vert in draw_list.vtx_buffer() {
                let vertex =
                    Vector3::new(vert.pos[0] - half_width, -vert.pos[1] + half_height, 1.0);
                let uv = Vector2::new(vert.uv[0], vert.uv[1]);
                let color = Color::from(vert.col);
                vertices.push(vertex);
                uvs.push(uv);
                colors.push(color);
                normals.push(Vector3::backward());
            }
            to = vertices.len() as u32;
        }

        // update mesh
        self.mesh.set_vertices(&vertices);
        self.mesh.set_normals(&normals);
        self.mesh.set_colors(&colors);
        self.mesh.set_uvs(&uvs);
        self.mesh.set_indices(&indices);
        self.mesh.update_if_needed()?;

        // render ui
        let cmd = self.device.command_buffer();

        self.framebuffer.with(|f| {
            // update world uniform
            f.world_uniform()
                .update(WorldData {
                    lights: [Default::default(); 4],
                    world_matrix: f.camera.matrix(),
                    camera_position: f.camera.transform.position,
                    time: 0.0,
                    cascade_splits: [0.0; 4],
                    light_matrices: [Matrix4::identity(); 4],
                    pcf: 0.0,
                })
                .expect("bad update");

            // begin render pass
            self.device
                .cmd_begin_render_pass(cmd, &f, [0.0, 0.0, 0.0, 0.0]);
            self.device.cmd_set_view(cmd, f.width(), f.height());
            self.device.cmd_set_line_width(cmd, 1.0);

            // bind resources
            self.device
                .cmd_bind_uniform(cmd, shader_layout, f.world_uniform());
            self.device.cmd_bind_shader(cmd, &self.shader);

            // render mesh
            let albedo_index = if let Some(texture) = &self.texture {
                texture.image_index()
            } else {
                0
            };
            self.device.cmd_push_constants(
                cmd,
                shader_layout,
                PushConstants {
                    model_matrix: Matrix4::identity(),
                    sampler_index: 0,
                    albedo_index,
                },
            );

            self.device.cmd_bind_mesh(cmd, &self.mesh);
            self.device.cmd_draw(cmd, self.mesh.index_count(), 0);

            self.device.cmd_end_render_pass(cmd);
            f.blit_to_texture(cmd);
        });

        self.drawn = true;

        Ok(())
    }

    pub(crate) fn set_font_texture(
        &mut self,
        image_uniform: &ImageUniform,
        texture: (Vec<u8>, u32, u32),
    ) -> Result<()> {
        self.texture = Some(Texture::new(
            &self.device,
            image_uniform,
            TextureOptions {
                data: &texture.0,
                width: texture.1,
                height: texture.2,
                format: ImageFormat::Rgba,
            },
        )?);

        Ok(())
    }

    pub(crate) fn resize(
        &self,
        image_uniform: &ImageUniform,
        width: u32,
        height: u32,
    ) -> Result<()> {
        self.framebuffer
            .with(|f| f.resize(width, height, image_uniform))
    }

    pub(crate) fn reset(&mut self) {
        self.drawn = false;
    }

    pub(crate) fn drawn(&self) -> bool {
        self.drawn
    }

    pub(crate) fn framebuffer(&self) -> &Ref<Framebuffer> {
        &self.framebuffer
    }
}
