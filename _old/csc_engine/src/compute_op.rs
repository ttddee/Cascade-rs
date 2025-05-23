use std::sync::Arc;

use vulkano::{
    buffer::{Buffer, BufferCreateInfo, BufferUsage},
    command_buffer::{
        CommandBufferBeginInfo, CommandBufferLevel, CommandBufferUsage, CopyBufferToImageInfo,
        RecordingCommandBuffer,
    },
    device::Queue,
    format::Format,
    image::{view::ImageView, Image, ImageCreateInfo, ImageType, ImageUsage},
    memory::allocator::{AllocationCreateInfo, MemoryTypeFilter},
    shader::ShaderModule,
    sync::GpuFuture,
    DeviceSize,
};

use crate::renderer::Allocators;

pub enum ComputeOp<'a> {
    LoadImage(LoadImageOp<'a>),
    ProcessImage(ProcessImageOp),
    SaveImage(SaveImageOp<'a>),
}

pub struct LoadImageOp<'a> {
    pub path: &'a str,
    pub gfx_queue: Arc<Queue>,
    pub allocators: &'a Allocators,
}

pub struct ProcessImageOp {
    shader: ShaderModule,
}

pub struct SaveImageOp<'a> {
    path: &'a str,
}

impl<'a> LoadImageOp<'a> {
    pub fn run(&self, result_image: Arc<ImageView>) -> Arc<ImageView> {
        let png_bytes = include_bytes!("../../assets/images/test.png").as_slice();
        let decoder = png::Decoder::new(png_bytes);
        let mut reader = decoder.read_info().unwrap();
        let info = reader.info();
        let extent = [info.width, info.height, 1];

        let upload_buffer = Buffer::new_slice(
            self.allocators.memory.clone(),
            BufferCreateInfo {
                usage: BufferUsage::TRANSFER_SRC,
                ..Default::default()
            },
            AllocationCreateInfo {
                memory_type_filter: MemoryTypeFilter::PREFER_HOST
                    | MemoryTypeFilter::HOST_SEQUENTIAL_WRITE,
                ..Default::default()
            },
            (extent[0] * extent[1] * 4) as DeviceSize,
        )
        .unwrap();

        let image = Image::new(
            self.allocators.memory.clone(),
            ImageCreateInfo {
                image_type: ImageType::Dim2d,
                format: Format::R8G8B8A8_SRGB,
                extent,
                usage: ImageUsage::TRANSFER_DST | ImageUsage::SAMPLED,
                ..Default::default()
            },
            AllocationCreateInfo::default(),
        )
        .unwrap();

        let texture = {
            reader
                .next_frame(&mut upload_buffer.write().unwrap())
                .unwrap();

            ImageView::new_default(image.clone()).unwrap()
        };

        let mut uploads = RecordingCommandBuffer::new(
            self.allocators.command_buffers.clone(),
            self.gfx_queue.queue_family_index(),
            CommandBufferLevel::Primary,
            CommandBufferBeginInfo {
                usage: CommandBufferUsage::OneTimeSubmit,
                ..Default::default()
            },
        )
        .unwrap();

        uploads
            .copy_buffer_to_image(CopyBufferToImageInfo::buffer_image(
                upload_buffer,
                image.clone(),
            ))
            .unwrap();

        uploads
            .end()
            .unwrap()
            .execute(self.gfx_queue.clone())
            .unwrap()
            .boxed();

        texture
    }
}

impl ProcessImageOp {
    pub fn run(&self, bg_image: Arc<ImageView>) -> Arc<ImageView> {
        // TODO
        bg_image
    }
}

impl<'a> SaveImageOp<'a> {
    pub fn run(&self, bg_image: Arc<ImageView>) -> Arc<ImageView> {
        // TODO
        bg_image
    }
}
