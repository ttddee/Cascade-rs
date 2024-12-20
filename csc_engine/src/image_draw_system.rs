// Copyright (c) 2017 The vulkano developers <=== !
// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT
// license <LICENSE-MIT or http://opensource.org/licenses/MIT>,
// at your option. All files in the project carrying such
// notice may not be copied, modified, or distributed except
// according to those terms.

use std::{convert::TryInto, sync::Arc};

use vulkano::{
    buffer::{Buffer, BufferContents, BufferCreateInfo, BufferUsage, Subbuffer},
    command_buffer::{
        allocator::CommandBufferAllocator, CommandBuffer, CommandBufferBeginInfo,
        CommandBufferInheritanceInfo, CommandBufferLevel, CommandBufferUsage,
        CopyBufferToImageInfo, RecordingCommandBuffer,
    },
    descriptor_set::{DescriptorSet, WriteDescriptorSet},
    device::Queue,
    format::Format,
    image::{
        sampler::{Filter, Sampler, SamplerAddressMode, SamplerCreateInfo},
        view::ImageView,
        Image, ImageCreateInfo, ImageType, ImageUsage,
    },
    memory::allocator::{AllocationCreateInfo, MemoryTypeFilter},
    pipeline::{
        graphics::{
            color_blend::{ColorBlendAttachmentState, ColorBlendState},
            input_assembly::{InputAssemblyState, PrimitiveTopology},
            multisample::MultisampleState,
            rasterization::RasterizationState,
            vertex_input::{Vertex, VertexDefinition},
            viewport::{Viewport, ViewportState},
            GraphicsPipelineCreateInfo,
        },
        layout::PipelineDescriptorSetLayoutCreateInfo,
        DynamicState, GraphicsPipeline, Pipeline, PipelineBindPoint, PipelineLayout,
        PipelineShaderStageCreateInfo,
    },
    render_pass::Subpass,
    sync::GpuFuture,
    DeviceSize,
};

use crate::{renderer::Allocators, shaders};

pub struct ImageDrawSystem {
    pub gfx_queue: Arc<Queue>,
    vertex_buffer: Subbuffer<[CsVertex]>,
    pipeline: Arc<GraphicsPipeline>,
    subpass: Subpass,
    command_buffer_allocator: Arc<dyn CommandBufferAllocator>,
    sampler: Arc<Sampler>,
}

impl ImageDrawSystem {
    pub fn new(
        gfx_queue: Arc<Queue>,
        subpass: Subpass,
        allocators: &Allocators,
    ) -> ImageDrawSystem {
        let vertex_buffer = Buffer::from_iter(
            allocators.memory.clone(),
            BufferCreateInfo {
                usage: BufferUsage::VERTEX_BUFFER,
                ..Default::default()
            },
            AllocationCreateInfo {
                memory_type_filter: MemoryTypeFilter::PREFER_DEVICE
                    | MemoryTypeFilter::HOST_RANDOM_ACCESS,
                ..Default::default()
            },
            [
                CsVertex {
                    position: [-1.0, -1.0],
                    uv: [0.0, 0.0],
                },
                CsVertex {
                    position: [-1.0, 1.0],
                    uv: [0.0, 1.0],
                },
                CsVertex {
                    position: [1.0, -1.0],
                    uv: [1.0, 0.0],
                },
                CsVertex {
                    position: [1.0, 1.0],
                    uv: [1.0, 1.0],
                },
            ],
        )
        .unwrap();

        let sampler = Sampler::new(
            gfx_queue.device().clone(),
            SamplerCreateInfo {
                mag_filter: Filter::Nearest,
                min_filter: Filter::Nearest,
                address_mode: [SamplerAddressMode::ClampToBorder; 3],
                ..Default::default()
            },
        )
        .unwrap();

        let pipeline = {
            let vs = shaders::vs::load(gfx_queue.device().clone())
                .expect("failed to create shader module")
                .entry_point("main")
                .unwrap();
            let fs = shaders::fs::load(gfx_queue.device().clone())
                .expect("failed to create shader module")
                .entry_point("main")
                .unwrap();

            let vertex_input_state = CsVertex::per_vertex().definition(&vs).unwrap();

            let stages = [
                PipelineShaderStageCreateInfo::new(vs),
                PipelineShaderStageCreateInfo::new(fs),
            ];

            let layout = PipelineLayout::new(
                gfx_queue.device().clone(),
                PipelineDescriptorSetLayoutCreateInfo::from_stages(&stages)
                    .into_pipeline_layout_create_info(gfx_queue.device().clone())
                    .unwrap(),
            )
            .unwrap();

            GraphicsPipeline::new(
                gfx_queue.device().clone(),
                None,
                GraphicsPipelineCreateInfo {
                    stages: stages.into_iter().collect(),
                    vertex_input_state: Some(vertex_input_state),
                    input_assembly_state: Some(InputAssemblyState {
                        topology: PrimitiveTopology::TriangleStrip,
                        ..Default::default()
                    }),
                    viewport_state: Some(ViewportState::default()),
                    rasterization_state: Some(RasterizationState::default()),
                    multisample_state: Some(MultisampleState::default()),
                    color_blend_state: Some(ColorBlendState::with_attachment_states(
                        subpass.num_color_attachments(),
                        ColorBlendAttachmentState::default(),
                    )),
                    dynamic_state: [DynamicState::Viewport].into_iter().collect(),
                    subpass: Some(subpass.clone().into()),
                    ..GraphicsPipelineCreateInfo::layout(layout)
                },
            )
            .unwrap()
        };

        ImageDrawSystem {
            gfx_queue,
            vertex_buffer,
            pipeline,
            subpass,
            command_buffer_allocator: allocators.command_buffers.clone(),
            sampler,
        }
    }

    pub fn draw(&self, allocators: &Allocators, image: Arc<ImageView>) -> Arc<CommandBuffer> {
        let layout = self.pipeline.layout().set_layouts().get(0).unwrap();

        let image_view = image; //Self::load_image(self.gfx_queue.clone(), allocators);
        let image_size = image_view.image().extent();

        let desc_set = DescriptorSet::new(
            allocators.descriptor.clone(),
            layout.clone(),
            [
                WriteDescriptorSet::sampler(0, self.sampler.clone()),
                WriteDescriptorSet::image_view(1, image_view),
            ],
            [],
        )
        .unwrap();

        let mut builder = RecordingCommandBuffer::new(
            self.command_buffer_allocator.clone(),
            self.gfx_queue.queue_family_index(),
            CommandBufferLevel::Secondary,
            CommandBufferBeginInfo {
                usage: CommandBufferUsage::MultipleSubmit,
                inheritance_info: Some(CommandBufferInheritanceInfo {
                    render_pass: Some(self.subpass.clone().into()),
                    ..Default::default()
                }),
                ..Default::default()
            },
        )
        .unwrap();

        unsafe {
            builder
                .bind_pipeline_graphics(self.pipeline.clone())
                .unwrap()
                .set_viewport(
                    0,
                    [Viewport {
                        offset: [0.0, 0.0],
                        extent: [image_size[0] as f32, image_size[1] as f32],
                        depth_range: 0.0..=1.0,
                    }]
                    .into_iter()
                    .collect(),
                )
                .unwrap()
                .bind_vertex_buffers(0, self.vertex_buffer.clone())
                .unwrap()
                .bind_descriptor_sets(
                    PipelineBindPoint::Graphics,
                    self.pipeline.layout().clone(),
                    0,
                    desc_set.clone(),
                )
                .unwrap()
                .draw(self.vertex_buffer.len() as u32, 1, 0, 0)
                .unwrap();
        }

        builder.end().unwrap()
    }
}

#[repr(C)]
#[derive(BufferContents, Vertex)]
struct CsVertex {
    #[format(R32G32_SFLOAT)]
    position: [f32; 2],
    #[format(R32G32_SFLOAT)]
    uv: [f32; 2],
}
