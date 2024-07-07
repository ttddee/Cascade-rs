#![allow(clippy::eq_op)]

use egui_winit_vulkano::{Gui, GuiConfig};
use vulkano::{
    image::{view::ImageView, Image, ImageCreateInfo, ImageType, ImageUsage},
    memory::allocator::AllocationCreateInfo,
    sync::{self, GpuFuture},
};
use vulkano_util::{
    context::{VulkanoConfig, VulkanoContext},
    renderer::DEFAULT_IMAGE_FORMAT,
    window::{VulkanoWindows, WindowDescriptor, WindowMode},
};
use winit::{
    event::{Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
};

use csc_engine::renderer::RenderPipeline;
use csc_ui::{dock::MainDock, main_menu, style::load_style};

pub fn main() {
    // Winit event loop
    let event_loop = EventLoop::new().unwrap();
    // Vulkano context
    let vulkano_context = VulkanoContext::new(VulkanoConfig::default());
    // Vulkano windows (create one)
    let mut windows = VulkanoWindows::default();
    let window_descriptor = WindowDescriptor {
        width: 1920.,
        height: 1080.,
        mode: WindowMode::Windowed,
        title: "Cascade Image Editor".to_string(),
        ..Default::default()
    };
    windows.create_window(&event_loop, &vulkano_context, &window_descriptor, |ci| {
        ci.image_format = vulkano::format::Format::B8G8R8A8_UNORM;
        ci.min_image_count = ci.min_image_count.max(2);
    });

    // Create gui as main render pass (no overlay means it clears the image each frame)
    let mut gui = {
        let renderer = windows.get_primary_renderer_mut().unwrap();
        Gui::new(
            &event_loop,
            renderer.surface(),
            renderer.graphics_queue(),
            renderer.swapchain_format(),
            GuiConfig::default(),
        )
    };

    // TODO: These dimensions should be as big as the viewer area
    // and get updated on window resize
    let scene_view_size = [1440, 720];

    // The image that the viewer gets rendered on
    let viewer_image = ImageView::new_default(
        Image::new(
            vulkano_context.memory_allocator().clone(),
            ImageCreateInfo {
                image_type: ImageType::Dim2d,
                format: DEFAULT_IMAGE_FORMAT,
                extent: [scene_view_size[0], scene_view_size[1], 1],
                array_layers: 1,
                usage: ImageUsage::SAMPLED | ImageUsage::COLOR_ATTACHMENT,
                ..Default::default()
            },
            AllocationCreateInfo::default(),
        )
        .unwrap(),
    )
    .unwrap();

    let mut scene_render_pipeline = RenderPipeline::new(
        vulkano_context.graphics_queue().clone(),
        DEFAULT_IMAGE_FORMAT,
        &vulkano_context,
    );

    let mut main_dock = MainDock::new(&mut gui, viewer_image.clone(), scene_view_size);

    load_style(&mut gui.context());

    event_loop.run(move |event, _, control_flow| {
        let renderer = windows.get_primary_renderer_mut().unwrap();

        match event {
            Event::WindowEvent { event, window_id } if window_id == renderer.window().id() => {
                // Update egui integration so the UI works!
                let _pass_events_to_app = !gui.update(&event);
                match event {
                    WindowEvent::Resized(_) => {
                        renderer.resize();
                    }
                    WindowEvent::ScaleFactorChanged { .. } => {
                        renderer.resize();
                    }
                    // WindowEvent::CloseRequested => {
                    //     *control_flow = ControlFlow::Exit;
                    // }
                    WindowEvent::RedrawRequested { .. } => {
                        gui.immediate_ui(|gui| {
                            main_menu::build_main_menu(&gui.context());

                            main_dock.show(gui.context());
                        });
                        // Acquire swapchain future
                        let before_future = match renderer.acquire() {
                            Ok(future) => future,
                            Err(vulkano::VulkanError::OutOfDate) => {
                                renderer.resize();
                                sync::now(vulkano_context.device().clone()).boxed()
                            }
                            Err(e) => panic!("Failed to acquire swapchain future: {}", e),
                        };
                        // Draw scene
                        let after_scene_draw =
                            scene_render_pipeline.render(before_future, viewer_image.clone());
                        // Render gui
                        let after_future =
                            gui.draw_on_image(after_scene_draw, renderer.swapchain_image_view());
                        // Present swapchain
                        renderer.present(after_future, true);
                    }
                    _ => {}
                }
            }
            // Event::WindowEvent(window_id) if window_id == window_id => {
            //     // Set immediate UI in redraw here
            //     // It's a closure giving access to egui context inside which you can call anything.
            //     // Here we're calling the layout of our `gui_state`.
            //     gui.immediate_ui(|gui| {
            //         main_menu::build_main_menu(&gui.context());

            //         main_dock.show(gui.context());
            //     });
            //     // Acquire swapchain future
            //     let before_future = match renderer.acquire() {
            //         Ok(future) => future,
            //         Err(vulkano::VulkanError::OutOfDate) => {
            //             renderer.resize();
            //             sync::now(vulkano_context.device().clone()).boxed()
            //         }
            //         Err(e) => panic!("Failed to acquire swapchain future: {}", e),
            //     };
            //     // Draw scene
            //     let after_scene_draw =
            //         scene_render_pipeline.render(before_future, viewer_image.clone());
            //     // Render gui
            //     let after_future =
            //         gui.draw_on_image(after_scene_draw, renderer.swapchain_image_view());
            //     // Present swapchain
            //     renderer.present(after_future, true);
            // }
            Event::AboutToWait => {
                renderer.window().request_redraw();
            }
            _ => (),
        }
    });
}
