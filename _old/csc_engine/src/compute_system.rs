use std::sync::Arc;

use vulkano::{
    buffer::{Buffer, BufferCreateInfo, BufferUsage},
    command_buffer::{
        allocator::CommandBufferAllocator, CommandBufferBeginInfo, CommandBufferLevel,
        CommandBufferUsage, RecordingCommandBuffer,
    },
    descriptor_set::{DescriptorSet, WriteDescriptorSet},
    device::Queue,
    image::view::ImageView,
    memory::allocator::{AllocationCreateInfo, MemoryTypeFilter},
    pipeline::{
        compute::ComputePipelineCreateInfo, layout::PipelineDescriptorSetLayoutCreateInfo,
        ComputePipeline, Pipeline, PipelineBindPoint, PipelineLayout,
        PipelineShaderStageCreateInfo,
    },
};

use crate::compute_op::{ComputeOp, LoadImageOp, SaveImageOp};
use crate::renderer::Allocators;
use crate::shaders;

pub struct ComputeSystem {
    compute_queue: Arc<Queue>,
    pipeline: Arc<ComputePipeline>,
    command_buffer_allocator: Arc<dyn CommandBufferAllocator>,
}

impl ComputeSystem {
    pub fn new(compute_queue: Arc<Queue>, allocators: &Allocators) -> ComputeSystem {
        let pipeline = {
            let cs = shaders::cs::load(compute_queue.device().clone())
                .unwrap()
                .entry_point("main")
                .unwrap();
            let stage = PipelineShaderStageCreateInfo::new(cs);
            let layout = PipelineLayout::new(
                compute_queue.device().clone(),
                PipelineDescriptorSetLayoutCreateInfo::from_stages([&stage])
                    .into_pipeline_layout_create_info(compute_queue.device().clone())
                    .unwrap(),
            )
            .unwrap();
            ComputePipeline::new(
                compute_queue.device().clone(),
                None,
                ComputePipelineCreateInfo::stage_layout(stage, layout),
            )
            .unwrap()
        };

        ComputeSystem {
            compute_queue,
            pipeline,
            command_buffer_allocator: allocators.command_buffers.clone(),
        }
    }

    pub fn execute(
        &self,
        queued_operations: &Vec<ComputeOp>,
        mut result_image: Arc<ImageView>,
    ) -> Arc<ImageView> {
        for op in queued_operations {
            match op {
                ComputeOp::LoadImage(load_image_op) => {
                    result_image = load_image_op.run(result_image.clone())
                }
                ComputeOp::ProcessImage(process_image_op) => {
                    result_image = process_image_op.run(result_image.clone())
                }
                ComputeOp::SaveImage(save_image_op) => {
                    result_image = save_image_op.run(result_image.clone())
                }
            };
        }

        result_image
    }
}
