#![allow(clippy::eq_op)]

use std::sync::Arc;

use egui_node_graph::GraphEditorState;
use egui_winit_vulkano::{Gui, GuiConfig};
use vulkano::{
    command_buffer::allocator::{
        StandardCommandBufferAllocator, StandardCommandBufferAllocatorCreateInfo,
    },
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

use csc_core::graph_model::NodeGraphState;
use csc_core::node_model::{CsImageType, MyNodeData, MyValueType, NodeType};
use csc_engine::renderer;
use csc_engine::{pipeline::RenderPipelineOld, renderer::RenderPipeline};
use csc_ui::{
    dock::MainDock, gui_state::GuiState, main_menu, node_graph, properties_panel::PropertiesPanel,
    style::load_style,
};

pub fn main() {
    // Winit event loop
    let event_loop = EventLoop::new();
    // Vulkano context
    let vulkano_context = VulkanoContext::new(VulkanoConfig::default());
    // Vulkano windows (create one)
    let mut windows = VulkanoWindows::default();
    let window_descriptor = WindowDescriptor {
        width: 1920.,
        height: 1080.,
        mode: WindowMode::Windowed,
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

    let scene_view_size = [1024, 512];

    // Create a simple image to which we'll draw the triangle scene
    let scene_image = ImageView::new_default(
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

    // Create our render pipeline
    let mut scene_render_pipeline = RenderPipeline::new(
        vulkano_context.graphics_queue().clone(),
        DEFAULT_IMAGE_FORMAT,
        &renderer::Allocators {
            command_buffers: Arc::new(StandardCommandBufferAllocator::new(
                vulkano_context.device().clone(),
                StandardCommandBufferAllocatorCreateInfo {
                    secondary_buffer_count: 32,
                    ..Default::default()
                },
            )),
            memory: vulkano_context.memory_allocator().clone(),
        },
    );
    // Create gui state (pass anything your state requires)
    //let mut gui_state = GuiState::new(&mut gui, scene_image.clone(), scene_view_size);

    // // Create the rendering pipeline
    // let mut render_pipeline = RenderPipeline::new(
    //     vulkano_context.graphics_queue().clone(),
    //     windows
    //         .get_primary_renderer_mut()
    //         .unwrap()
    //         .swapchain_format(),
    //     vulkano_context.memory_allocator().clone(),
    // );
    // // Create gui subpass
    // let mut gui_subpass = Gui::new_with_subpass(
    //     &event_loop,
    //     windows.get_primary_renderer_mut().unwrap().surface(),
    //     windows.get_primary_renderer_mut().unwrap().graphics_queue(),
    //     render_pipeline.gui_pass(),
    //     windows
    //         .get_primary_renderer_mut()
    //         .unwrap()
    //         .swapchain_format(),
    //     GuiConfig::default(),
    // );

    let mut graph_state: GraphEditorState<
        MyNodeData,
        ImageType,
        MyValueType,
        NodeType,
        NodeGraphState,
    > = GraphEditorState::new(1.0);

    let mut user_state = NodeGraphState::default();

    let mut properties_panel = PropertiesPanel::default();

    let mut main_dock = MainDock::new(&mut gui, scene_image.clone(), scene_view_size);

    //load_style(&mut gui_subpass.context());

    // Create gui state (pass anything your state requires)
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
                    WindowEvent::CloseRequested => {
                        *control_flow = ControlFlow::Exit;
                    }
                    _ => {}
                }
            }
            Event::RedrawRequested(window_id) if window_id == window_id => {
                // Set immediate UI in redraw here
                // It's a closure giving access to egui context inside which you can call anything.
                // Here we're calling the layout of our `gui_state`.
                gui.immediate_ui(|gui| {
                    main_menu::build_main_menu(&gui.context());

                    main_dock.show(gui.context());
                });
                // Render UI
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
                    scene_render_pipeline.render(before_future, scene_image.clone());
                // Render gui
                let after_future =
                    gui.draw_on_image(after_scene_draw, renderer.swapchain_image_view());
                // Present swapchain
                renderer.present(after_future, true);

                // Set immediate UI in redraw here
                // gui_subpass.immediate_ui(|gui| {
                //     main_menu::build_main_menu(&gui.context());

                //     main_dock.show(gui.context());

                //     // let ctx = gui.context();

                //     // properties_panel.show(&ctx, &mut graph_state);

                //     // node_graph::build_node_graph(&ctx, &mut graph_state, &mut user_state)
                // });
                // // Render
                // // Acquire swapchain future
                // let before_future = renderer.acquire().unwrap();
                // // Render scene
                // // Render gui
                // let after_future = render_pipeline.render(
                //     before_future,
                //     renderer.swapchain_image_view(),
                //     &mut gui_subpass,
                // );
                // // Present swapchain
                // renderer.present(after_future, true);
            }
            Event::MainEventsCleared => {
                renderer.window().request_redraw();
            }
            _ => (),
        }
    });
}
