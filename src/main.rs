#![allow(clippy::eq_op)]

use egui_node_graph::GraphEditorState;
use egui_winit_vulkano::{Gui, GuiConfig};
use vulkano_util::{
    context::{VulkanoConfig, VulkanoContext},
    window::{VulkanoWindows, WindowDescriptor, WindowMode},
};
use winit::{
    event::{Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
};

use csc_core::graph_model::NodeGraphState;
use csc_core::node_model::{ImageType, MyNodeData, MyValueType, NodeType};
use csc_engine::pipeline::RenderPipeline;
use csc_ui::{main_menu, node_graph, properties_panel::PropertiesPanel, style::load_style};

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
    // Create the rendering pipeline
    let mut gui_pipeline = RenderPipeline::new(
        vulkano_context.graphics_queue().clone(),
        windows
            .get_primary_renderer_mut()
            .unwrap()
            .swapchain_format(),
        vulkano_context.memory_allocator().clone(),
    );
    // Create gui subpass
    let mut gui = Gui::new_with_subpass(
        &event_loop,
        windows.get_primary_renderer_mut().unwrap().surface(),
        windows.get_primary_renderer_mut().unwrap().graphics_queue(),
        gui_pipeline.gui_pass(),
        windows
            .get_primary_renderer_mut()
            .unwrap()
            .swapchain_format(),
        GuiConfig::default(),
    );

    let mut graph_state: GraphEditorState<
        MyNodeData,
        ImageType,
        MyValueType,
        NodeType,
        NodeGraphState,
    > = GraphEditorState::new(1.0);

    let mut user_state = NodeGraphState::default();

    let mut properties_panel = PropertiesPanel::default();

    load_style(&mut gui.context());

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
                gui.immediate_ui(|gui| {
                    let ctx = gui.context();

                    main_menu::build_main_menu(&ctx);

                    properties_panel.show(&ctx, &mut graph_state);

                    node_graph::build_node_graph(&ctx, &mut graph_state, &mut user_state)
                });
                // Render
                // Acquire swapchain future
                let before_future = renderer.acquire().unwrap();
                // Render scene

                // Render gui
                let after_future =
                    gui_pipeline.render(before_future, renderer.swapchain_image_view(), &mut gui);
                // Present swapchain
                renderer.present(after_future, true);
            }
            Event::MainEventsCleared => {
                renderer.window().request_redraw();
            }
            _ => (),
        }
    });
}
