use std::ops::RangeInclusive;

use egui::SidePanel;

use egui_node_graph::GraphEditorState;

use csc_core::graph_model::NodeGraphState;
use csc_core::node_model::{ImageType, MyNodeData, MyValueType, NodeType};
use csc_core::node_property::NodeProperty;

pub fn build_properties_panel(
    context: &egui::Context,
    graph_state: &mut GraphEditorState<
        MyNodeData,
        ImageType,
        MyValueType,
        NodeType,
        NodeGraphState,
    >,
) {
    SidePanel::right("properties_panel")
        .default_width(300.)
        .show(context, |ui| {
            ui.separator();
            if let Some(node_id) = graph_state.active_node {
                let active_node = &mut graph_state.graph.nodes[node_id];
                let node_type = active_node.user_data.node_type;
                if active_node.user_data.node_type == NodeType::Blur {
                    ui.label(egui::RichText::new(node_type.name()).strong());
                    ui.separator();

                    egui::ScrollArea::vertical().show(ui, |ui| {
                        egui::Grid::new("properties_grid")
                            .num_columns(2)
                            .spacing([40.0, 8.0])
                            .striped(true)
                            .show(ui, |ui| {
                                for property in &mut active_node.user_data.node_properties {
                                    match property {
                                        NodeProperty::Float(data) => {
                                            let range =
                                                RangeInclusive::new(*data.min(), *data.max());
                                            ui.add(egui::Label::new(data.name().clone()));
                                            ui.add(egui::Slider::new(data.value(), range));
                                            ui.end_row();
                                        }
                                        NodeProperty::Choice(data) => {
                                            let length = data.choices().len();
                                            let choices = data.choices().clone();
                                            ui.add(egui::Label::new(data.name()));
                                            egui::ComboBox::from_id_source("combobox").show_index(
                                                ui,
                                                data.index(),
                                                length,
                                                |i| &choices[i],
                                            );
                                            ui.end_row();
                                        }
                                        _ => {}
                                    }
                                }
                            });
                    });
                }
            }
        });
}
