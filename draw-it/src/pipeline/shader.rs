// Oliver Berzs
// https://github.com/OllieBerzs/draw-it

// Shader - GPU program for execution in the renderer

use ash::vk;
use serde::Deserialize;
use std::ffi::CString;
use std::sync::Arc;

use super::CullMode;
use super::DepthMode;
use super::PolygonMode;
use super::ShaderLayout;
use crate::device::Device;
use crate::error::Result;
use crate::image::Framebuffer;
use crate::image::Msaa;
use crate::mesh::Vertex;

pub struct Shader {
    handle: vk::Pipeline,
    device: Arc<Device>,
}

#[derive(Debug, Copy, Clone)]
pub struct ShaderOptions {
    pub depth_mode: DepthMode,
    pub polygon_mode: PolygonMode,
    pub cull_mode: CullMode,
}

#[derive(Deserialize)]
struct ShaderFile {
    vert: Vec<u8>,
    frag: Vec<u8>,
}

impl Shader {
    pub(crate) fn new(
        device: &Arc<Device>,
        framebuffer: &Framebuffer,
        layout: &ShaderLayout,
        source: &[u8],
        options: ShaderOptions,
    ) -> Result<Self> {
        let data: ShaderFile = bincode::deserialize(source)?;

        let vert_module = device.create_shader_module(&data.vert)?;
        let frag_module = device.create_shader_module(&data.frag)?;
        let entry_point = CString::new("main").expect("bad code");

        // configure vertex stage
        let vs_stage_info = vk::PipelineShaderStageCreateInfo::builder()
            .stage(vk::ShaderStageFlags::VERTEX)
            .module(vert_module)
            .name(&entry_point)
            .build();

        // configure fragment stage
        let fs_stage_info = vk::PipelineShaderStageCreateInfo::builder()
            .stage(vk::ShaderStageFlags::FRAGMENT)
            .module(frag_module)
            .name(&entry_point)
            .build();

        // configure vertex input state
        let binding_descriptions = [Vertex::binding_description()];
        let attribute_descriptions = Vertex::attribute_descriptions();
        let vertex_input_info = vk::PipelineVertexInputStateCreateInfo::builder()
            .vertex_binding_descriptions(&binding_descriptions)
            .vertex_attribute_descriptions(&attribute_descriptions)
            .build();

        // configure assembly input state
        let assembly_input_info = vk::PipelineInputAssemblyStateCreateInfo::builder()
            .topology(options.polygon_mode.topology())
            .primitive_restart_enable(false);

        // configure viewport state
        let viewport = [vk::Viewport {
            x: 0.0,
            y: 1.0,
            width: 1.0,
            height: -1.0,
            min_depth: 0.0,
            max_depth: 1.0,
        }];

        let scissor = [vk::Rect2D {
            offset: vk::Offset2D { x: 0, y: 0 },
            extent: vk::Extent2D {
                width: 1,
                height: 1,
            },
        }];

        let viewport_state = vk::PipelineViewportStateCreateInfo::builder()
            .viewports(&viewport)
            .scissors(&scissor)
            .build();

        // configure rasterization state
        let rasterizer_state = vk::PipelineRasterizationStateCreateInfo::builder()
            .depth_clamp_enable(false)
            .rasterizer_discard_enable(false)
            .depth_bias_enable(false)
            .front_face(vk::FrontFace::CLOCKWISE)
            .cull_mode(options.cull_mode.flag())
            .polygon_mode(options.polygon_mode.polygon())
            .line_width(1.0);

        // configure msaa state
        let msaa = if framebuffer.multisampled() {
            device.msaa()
        } else {
            Msaa::Disabled
        };

        let multisampling = vk::PipelineMultisampleStateCreateInfo::builder()
            .sample_shading_enable(false)
            .rasterization_samples(msaa.flag());

        // configure depth stencil state
        let depth_stencil_state = vk::PipelineDepthStencilStateCreateInfo::builder()
            .depth_test_enable(options.depth_mode.test())
            .depth_write_enable(options.depth_mode.write())
            .depth_compare_op(vk::CompareOp::LESS)
            .depth_bounds_test_enable(false)
            .stencil_test_enable(false);

        // configure color blend state
        let color_blend_attachment = [vk::PipelineColorBlendAttachmentState::builder()
            .color_write_mask(
                vk::ColorComponentFlags::R
                    | vk::ColorComponentFlags::G
                    | vk::ColorComponentFlags::B
                    | vk::ColorComponentFlags::A,
            )
            .blend_enable(true)
            .src_color_blend_factor(vk::BlendFactor::SRC_ALPHA)
            .dst_color_blend_factor(vk::BlendFactor::ONE_MINUS_SRC_ALPHA)
            .color_blend_op(vk::BlendOp::ADD)
            .src_alpha_blend_factor(vk::BlendFactor::SRC_ALPHA)
            .dst_alpha_blend_factor(vk::BlendFactor::ONE_MINUS_SRC_ALPHA)
            .alpha_blend_op(vk::BlendOp::ADD)
            .build()];

        let color_blending = vk::PipelineColorBlendStateCreateInfo::builder()
            .attachments(&color_blend_attachment)
            .logic_op_enable(false)
            .build();

        // configure dynamic state
        let dynamic_states = [
            vk::DynamicState::LINE_WIDTH,
            vk::DynamicState::SCISSOR,
            vk::DynamicState::VIEWPORT,
        ];
        let dynamic_state = vk::PipelineDynamicStateCreateInfo::builder()
            .dynamic_states(&dynamic_states)
            .build();

        // create pipeline
        let stages = [vs_stage_info, fs_stage_info];
        let pipeline_info = vk::GraphicsPipelineCreateInfo::builder()
            .stages(&stages)
            .vertex_input_state(&vertex_input_info)
            .input_assembly_state(&assembly_input_info)
            .viewport_state(&viewport_state)
            .rasterization_state(&rasterizer_state)
            .multisample_state(&multisampling)
            .color_blend_state(&color_blending)
            .depth_stencil_state(&depth_stencil_state)
            .dynamic_state(&dynamic_state)
            .layout(layout.handle())
            .render_pass(framebuffer.render_pass())
            .subpass(0)
            .build();

        let handle = device.create_pipeline(pipeline_info)?;

        device.destroy_shader_module(vert_module);
        device.destroy_shader_module(frag_module);

        Ok(Self {
            handle,
            device: Arc::clone(device),
        })
    }

    pub(crate) fn handle(&self) -> vk::Pipeline {
        self.handle
    }
}

impl Drop for Shader {
    fn drop(&mut self) {
        self.device.destroy_pipeline(self.handle);
    }
}

impl PartialEq for Shader {
    fn eq(&self, other: &Self) -> bool {
        self.handle == other.handle
    }
}

impl Default for ShaderOptions {
    fn default() -> Self {
        Self {
            depth_mode: DepthMode::TestAndWrite,
            polygon_mode: PolygonMode::FilledTriangles,
            cull_mode: CullMode::Back,
        }
    }
}
