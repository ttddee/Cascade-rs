use std::sync::Arc;

use egui::{load::SizedTexture, Context, ImageSource};
use egui_winit_vulkano::{egui, Gui, GuiConfig};

use vulkano::{
    command_buffer::allocator::{
        StandardCommandBufferAllocator, StandardCommandBufferAllocatorCreateInfo,
    },
    format::Format,
    image::{view::ImageView, Image, ImageCreateInfo, ImageType, ImageUsage},
    memory::allocator::AllocationCreateInfo,
    sync::{self, GpuFuture},
};

pub struct GuiState {
    scene_texture_id: egui::TextureId,
    scene_view_size: [u32; 2],
}

impl GuiState {
    pub fn new(gui: &mut Gui, scene_image: Arc<ImageView>, scene_view_size: [u32; 2]) -> GuiState {
        GuiState {
            scene_texture_id: gui.register_user_image_view(scene_image, Default::default()),
            scene_view_size,
        }
    }

    /// Defines the layout of our UI
    pub fn layout(&mut self, egui_context: Context) {
        let GuiState {
            scene_view_size,
            scene_texture_id,
            ..
        } = self;

        egui::Window::new("Scene")
            .resizable(true)
            .vscroll(true)
            .open(&mut true)
            .show(&egui_context, |ui| {
                ui.image(ImageSource::Texture(SizedTexture::new(
                    *scene_texture_id,
                    [scene_view_size[0] as f32, scene_view_size[1] as f32],
                )));
            });
    }
}
