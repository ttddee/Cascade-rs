use std::sync::Arc;

use vulkano::{
    buffer::{Buffer, BufferCreateInfo, BufferUsage},
    command_buffer::{
        allocator::StandardCommandBufferAllocator, AutoCommandBufferBuilder,
        CommandBufferInheritanceInfo, CommandBufferUsage, SecondaryAutoCommandBuffer,
    },
    descriptor_set::{PersistentDescriptorSet, WriteDescriptorSet},
    device::Queue,
    memory::allocator::{AllocationCreateInfo, MemoryTypeFilter},
    pipeline::{
        compute::ComputePipelineCreateInfo, layout::PipelineDescriptorSetLayoutCreateInfo,
        ComputePipeline, Pipeline, PipelineBindPoint, PipelineLayout,
        PipelineShaderStageCreateInfo,
    },
    render_pass::Subpass,
};

use crate::renderer::Allocators;

pub struct ComputeSystem {
    compute_queue: Arc<Queue>,
    subpass: Subpass,
    pipeline: Arc<ComputePipeline>,
    command_buffer_allocator: Arc<StandardCommandBufferAllocator>,
    descriptor_set: Arc<PersistentDescriptorSet>,
}

impl ComputeSystem {
    pub fn new(
        compute_queue: Arc<Queue>,
        subpass: Subpass,
        allocators: &Allocators,
    ) -> ComputeSystem {
        let pipeline = {
            mod cs {
                vulkano_shaders::shader! {
                    ty: "compute",
                    src: r"
                        #version 450
    
                        layout(local_size_x = 64, local_size_y = 1, local_size_z = 1) in;
    
                        layout(set = 0, binding = 0) buffer Data {
                            uint data[];
                        };
    
                        void main() {
                            uint idx = gl_GlobalInvocationID.x;
                            data[idx] *= 12;
                        }
                    ",
                }
            }
            let cs = cs::load(compute_queue.device().clone())
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

        // We start by creating the buffer that will store the data.
        let data_buffer = Buffer::from_iter(
            allocators.memory.clone(),
            BufferCreateInfo {
                usage: BufferUsage::STORAGE_BUFFER,
                ..Default::default()
            },
            AllocationCreateInfo {
                memory_type_filter: MemoryTypeFilter::PREFER_DEVICE
                    | MemoryTypeFilter::HOST_RANDOM_ACCESS,
                ..Default::default()
            },
            // Iterator that produces the data.
            0..65536u32,
        )
        .unwrap();

        // In order to let the shader access the buffer, we need to build a *descriptor set* that
        // contains the buffer.
        //
        // The resources that we bind to the descriptor set must match the resources expected by the
        // pipeline which we pass as the first parameter.
        //
        // If you want to run the pipeline on multiple different buffers, you need to create multiple
        // descriptor sets that each contain the buffer you want to run the shader on.
        let layout = pipeline.layout().set_layouts()[0].clone();
        let descriptor_set = PersistentDescriptorSet::new(
            &allocators.descriptor,
            layout.clone(),
            [WriteDescriptorSet::buffer(0, data_buffer.clone())],
            [],
        )
        .unwrap();

        ComputeSystem {
            compute_queue,
            subpass,
            pipeline,
            command_buffer_allocator: allocators.command_buffers.clone(),
            descriptor_set,
        }
    }

    pub fn execute(
        &self,
        allocators: &Allocators,
    ) -> Arc<SecondaryAutoCommandBuffer<Arc<StandardCommandBufferAllocator>>> {
        let mut builder = AutoCommandBufferBuilder::secondary(
            &self.command_buffer_allocator,
            self.compute_queue.queue_family_index(),
            CommandBufferUsage::MultipleSubmit,
            CommandBufferInheritanceInfo {
                render_pass: Some(self.subpass.clone().into()),
                ..Default::default()
            },
        )
        .unwrap();

        builder
            .bind_pipeline_compute(self.pipeline.clone())
            .unwrap()
            .bind_descriptor_sets(
                PipelineBindPoint::Compute,
                self.pipeline.layout().clone(),
                0,
                self.descriptor_set.clone(),
            )
            .unwrap();

        // The command buffer only does one thing: execute the compute pipeline. This is called a
        // *dispatch* operation.
        builder.dispatch([1024, 1, 1]).unwrap();

        // Finish building the command buffer by calling `build`.
        builder.build().unwrap()
    }
}
