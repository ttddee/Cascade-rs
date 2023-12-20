use std::ops::RangeInclusive;

use egui::{SidePanel, Slider};

use egui_node_graph::GraphEditorState;

use csc_core::graph_model::MyGraphState;
use csc_core::node_model::{ImageType, MyNodeData, MyValueType, NodeType};
use csc_core::node_property::NodeProperty;

pub fn build_properties_panel(
    ctx: &egui::Context,
    graph_editor_state: &mut GraphEditorState<
        MyNodeData,
        ImageType,
        MyValueType,
        NodeType,
        MyGraphState,
    >,
) {
    SidePanel::right("properties_panel")
        .default_width(450.)
        .show(&ctx, |ui| {
            if let Some(node_id) = graph_editor_state.active_node {
                let active_node = &mut graph_editor_state.graph.nodes[node_id];
                let node_type = active_node.user_data.node_type;
                if active_node.user_data.node_type == NodeType::Blur {
                    ui.label(node_type.name().as_ref());
                    ui.separator();

                    for property in &mut active_node.user_data.node_properties {
                        match property {
                            NodeProperty::Float(data) => {
                                let range = RangeInclusive::new(*data.min(), *data.max());
                                let name = data.name().clone();
                                ui.add(egui::Slider::new(data.value(), range).text(name));
                            }
                            NodeProperty::Choice(data) => {
                                let length = data.choices().len();
                                let choices = data.choices().clone();
                                egui::ComboBox::from_label(data.name()).show_index(
                                    ui,
                                    data.index(),
                                    length,
                                    |i| &choices[i],
                                );
                            }
                            _ => {}
                        }
                    }
                }
            }
        });
}
