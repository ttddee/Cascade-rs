
#![allow(clippy::eq_op)]

use std::collections::HashMap;

use csc_core::node_property::{NodeProperty, PropertyData};
use csc_engine::pipeline::RenderPipeline;
use csc_core::node_model::{ AllMyNodeTemplates, MyResponse, NodeType, MyNodeData, ImageType, MyValueType };
use csc_core::graph_model::{ MyGraphState };

use egui::{epaint::Shadow, style::Margin, vec2, Align, Align2, Color32, Frame, Rounding, Window, SidePanel };
use egui_winit_vulkano::{Gui, GuiConfig};
use vulkano_util::{
    context::{VulkanoConfig, VulkanoContext},
    window::{VulkanoWindows, WindowDescriptor, WindowMode},
};
use winit::{
    event::{Event, WindowEvent, MouseScrollDelta},
    event_loop::{ControlFlow, EventLoop},
};
use egui_node_graph::{ GraphEditorState, NodeResponse };

pub fn main() {
    // Winit event loop
    let event_loop = EventLoop::new();
    // Vulkano context
    let context = VulkanoContext::new(VulkanoConfig::default());
    // Vulkano windows (create one)
    let mut windows = VulkanoWindows::default();
    let window_descriptor = WindowDescriptor { 
        width: 1920.,
        height: 1080.,
        mode: WindowMode::Windowed, ..Default::default() };
    windows.create_window(&event_loop, &context, &window_descriptor, |ci| {
        ci.image_format = vulkano::format::Format::B8G8R8A8_UNORM;
        ci.min_image_count = ci.min_image_count.max(2);
    });
    // Create the rendering pipeline
    let mut gui_pipeline = RenderPipeline::new(
        context.graphics_queue().clone(),
        windows.get_primary_renderer_mut().unwrap().swapchain_format(),
        context.memory_allocator(),
    );
    // Create gui subpass
    let mut gui = Gui::new_with_subpass(
        &event_loop,
        windows.get_primary_renderer_mut().unwrap().surface(),
        windows.get_primary_renderer_mut().unwrap().graphics_queue(),
        gui_pipeline.gui_pass(),
        windows.get_primary_renderer_mut().unwrap().swapchain_format(),
        GuiConfig::default(),
    );

    let mut graph_editor_state: GraphEditorState<MyNodeData, ImageType, MyValueType, NodeType, MyGraphState> = GraphEditorState::new(0.0);
    let mut user_state = MyGraphState::default();

    // Create gui state (pass anything your state requires)
    event_loop.run(move |event, _, control_flow| {
        let renderer = windows.get_primary_renderer_mut().unwrap();
        match event {
            Event::WindowEvent { event, window_id } if window_id == renderer.window().id() => {
                // Update egui integration so the UI works!
                let _pass_events_to_game = !gui.update(&event);
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
                    WindowEvent::KeyboardInput { device_id: _, input: _, is_synthetic: false } => {
                        // if state == ElementState::Pressed && !event.repeat {
                        //     match event.key_without_modifiers().as_ref() {
                        //         Key::Character("1") => {
                        //             if modifiers.shift_key() {
                        //                 println!("Shift + 1 | logical_key: {:?}", event.logical_key);
                        //             } else {
                        //                 println!("1");
                        //             }
                        //         }
                        //         _ => (),
                        //     }
                        // }

                    }
                    // WindowEvent::MouseInput { device_id: _, state: _, button, modifiers: _} => {
                    //     //println!("Click on {:?} button", button);
                    // }
                    // WindowEvent::MouseWheel { device_id: _, delta, phase: _, modifiers: _ } => {
                    //     match delta {
                    //         MouseScrollDelta::LineDelta(_, y) => {
                    //             //println!("Y Delta {:?}", y);
                    //             if y > 0.0 {
                    //                 zoom += 0.1;
                    //             }
                    //             else {
                    //                 zoom -= 0.1;
                    //             }
                    //             zoom = zoom.max(-5.0);
                    //             zoom = zoom.min(5.0);
                    //         }
                    //         _ => (),
                    //     }
                    //     //println!("{:?}", delta);
                        
                    // }
                    _ => (),
                }
            }
            Event::RedrawRequested(window_id) if window_id == window_id => {
                // Set immediate UI in redraw here
                gui.immediate_ui(|gui| {
                    let ctx = gui.context();
                    Window::new("Transparent Window")
                        .anchor(Align2([Align::RIGHT, Align::TOP]), vec2(-545.0, 500.0))
                        .resizable(false)
                        .default_width(300.0)
                        .frame(
                            Frame::none()
                                .fill(Color32::from_white_alpha(125))
                                .shadow(Shadow {
                                    extrusion: 8.0,
                                    color: Color32::from_black_alpha(125),
                                })
                                .rounding(Rounding::same(5.0))
                                .inner_margin(Margin::same(10.0)),
                        )
                        .show(&ctx, |ui| {
                            ui.colored_label(Color32::BLACK, "Content :)");
                        });
                        egui::TopBottomPanel::top("main_menu").show(&ctx, |ui| {
                            egui::menu::bar(ui, |ui| {
                                egui::widgets::global_dark_light_mode_switch(ui);
                            });
                        });
                        SidePanel::right("properties_panel").default_width(300.).show(&ctx, |ui| {
               
                            if let Some(node_id) = graph_editor_state.active_node {
                                ui.label("active node");
                                
                                let active_node = &graph_editor_state.graph.nodes[node_id];
                                if active_node.user_data.node_type == NodeType::Blur {
                                    ui.label("blur");
                                    
                                    if let NodeProperty::Float(values) = &active_node.user_data.node_properties[0] {
                                        ui.label(values.value().to_string());
                                    }
                                }
                            }
                         });

                        // -------- Node Graph
                        let graph_response = egui::TopBottomPanel::bottom("nodegraph_panel").min_height(400.).resizable(true)
                            .show(&ctx, |ui| {
                                
                                graph_editor_state.draw_graph_editor(
                                    ui,
                                    AllMyNodeTemplates,
                                    &mut user_state,
                                    Vec::default(),
                                )
                            })
                            .inner;
                        for node_response in graph_response.node_responses {
                            // Here, we ignore all other graph events. But you may find
                            // some use for them. For example, by playing a sound when a new
                            // connection is created
                            // if let NodeResponse::User(user_event) = node_response {
                            //     match user_event {
                            //         MyResponse::SetActiveNode(node) => user_state.active_node = Some(node),
                            //         MyResponse::ClearActiveNode => user_state.active_node = None,
                            //     }
                            // }
                            //if let NodeResponse::User()
                        }
                
                        // if let Some(node) = user_state.active_node {
                        //     if graph_editor_state.graph.nodes.contains_key(node) {
                        //         // let text = match evaluate_node(&graph_editor_state.graph, node, &mut HashMap::new()) {
                        //         //     Ok(value) => format!("The result is: {:?}", value),
                        //         //     Err(err) => format!("Execution error: {}", err),
                        //         // };
                        //         // ctx.debug_painter().text(
                        //         //     egui::pos2(10.0, 35.0),
                        //         //     egui::Align2::LEFT_TOP,
                        //         //     text,
                        //         //     TextStyle::Button.resolve(&ctx.style()),
                        //         //     egui::Color32::WHITE,
                        //         // );
                        //     } else {
                        //         user_state.active_node = None;
                        //     }
                        // }
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

