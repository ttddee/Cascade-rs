use std::sync::Arc;

use cgmath::{Matrix4, SquareMatrix};
use vulkano::{
    command_buffer::allocator::{
        StandardCommandBufferAllocator, StandardCommandBufferAllocatorCreateInfo,
    },
    descriptor_set::allocator::StandardDescriptorSetAllocator,
    device::Queue,
    format::Format,
    image::view::ImageView,
    memory::allocator::StandardMemoryAllocator,
    sync::GpuFuture,
};
use vulkano_util::context::VulkanoContext;

use crate::{
    frame_system::{FrameSystem, Pass},
    image_draw_system::ImageDrawSystem,
};

#[derive(Clone)]
pub struct Allocators {
    pub command_buffers: Arc<StandardCommandBufferAllocator>,
    pub memory: Arc<StandardMemoryAllocator>,
    pub descriptor: Arc<StandardDescriptorSetAllocator>,
}

pub struct RenderPipeline {
    frame_system: FrameSystem,
    //draw_pipeline: TriangleDrawSystem,
    image_draw_system: ImageDrawSystem,
    allocators: Allocators,
}

impl RenderPipeline {
    pub fn new(queue: Arc<Queue>, image_format: Format, vulkano_context: &VulkanoContext) -> Self {
        let allocators = Allocators {
            command_buffers: Arc::new(StandardCommandBufferAllocator::new(
                queue.device().clone(),
                StandardCommandBufferAllocatorCreateInfo {
                    secondary_buffer_count: 32,
                    ..Default::default()
                },
            )),
            memory: vulkano_context.memory_allocator().clone(),
            descriptor: Arc::new(StandardDescriptorSetAllocator::new(
                queue.device().clone(),
                Default::default(),
            )),
        };

        let frame_system = FrameSystem::new(queue.clone(), image_format, allocators.clone());
        //let draw_pipeline =
        //TriangleDrawSystem::new(queue, frame_system.deferred_subpass(), &allocators);
        let image_draw_system =
            ImageDrawSystem::new(queue, frame_system.deferred_subpass(), &allocators);

        Self {
            frame_system,
            //draw_pipeline,
            image_draw_system,
            allocators,
        }
    }

    /// Renders the pass for scene on scene images
    pub fn render(
        &mut self,
        before_future: Box<dyn GpuFuture>,
        image: Arc<ImageView>,
    ) -> Box<dyn GpuFuture> {
        let mut frame = self.frame_system.frame(
            before_future,
            // Notice that final image is now scene image
            image.clone(),
            Matrix4::identity(),
        );
        let dims = image.image().extent();
        // Draw each render pass that's related to scene
        let mut after_future = None;
        while let Some(pass) = frame.next_pass() {
            match pass {
                Pass::Deferred(mut draw_pass) => {
                    let cb = Arc::new(
                        self.image_draw_system
                            .draw([dims[0], dims[1]], &self.allocators),
                    );
                    draw_pass.execute(cb);
                }
                Pass::Finished(af) => {
                    after_future = Some(af);
                }
            }
        }
        after_future
            .unwrap()
            .then_signal_fence_and_flush()
            .unwrap()
            .boxed()
    }
}
