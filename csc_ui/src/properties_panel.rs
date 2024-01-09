use std::ops::RangeInclusive;

use egui::{style, SidePanel, Ui};

use egui_node_graph::GraphEditorState;

use csc_core::graph_model::NodeGraphState;
use csc_core::node_model::{ImageType, MyNodeData, MyValueType, NodeType};
use csc_core::node_property::{ChoiceData, NodeProperty, NumberData, StringListData};

use crate::style::COLOR_BG_DARK;

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
                ui.with_layout(egui::Layout::top_down(egui::Align::LEFT), |ui| {
                    ui.label(egui::RichText::new(node_type.name()).strong());
                    ui.separator();

                    for property in &mut active_node.user_data.node_properties {
                        match property {
                            NodeProperty::Float(data) => {
                                show_float_property(ui, data);
                            }
                            NodeProperty::Choice(data) => {
                                show_choice_property(ui, data);
                            }
                            NodeProperty::StringList(data) => {
                                show_string_list_property(ui, data);
                            }
                            _ => {}
                        }
                        ui.add_space(2.);
                    }
                });
            }
        });
}

fn show_float_property(ui: &mut Ui, data: &mut NumberData<f32>) {
    let range = RangeInclusive::new(*data.min(), *data.max());
    ui.with_layout(egui::Layout::left_to_right(egui::Align::TOP), |ui| {
        ui.add(egui::Label::new(data.name().clone()));
        ui.add(egui::Slider::new(data.value(), range));
    });
}

fn show_choice_property(ui: &mut Ui, data: &mut ChoiceData) {
    let length = data.choices().len();
    let choices = data.choices().clone();
    ui.with_layout(egui::Layout::left_to_right(egui::Align::TOP), |ui| {
        ui.add(egui::Label::new(data.name()));
        egui::ComboBox::from_id_source("combobox")
            .show_index(ui, data.index(), length, |i| &choices[i]);
    });
}

fn show_string_list_property(ui: &mut Ui, data: &mut StringListData) {
    ui.button("Load").clicked();
    ui.separator();
    egui::Frame::none()
        .fill(COLOR_BG_DARK)
        .inner_margin(5.)
        .show(ui, |ui| {
            if ui
                .add(egui::Label::new("click me").sense(egui::Sense::click()))
                .clicked()
            { /* … */ }
            // Expand frame horizontally.
            ui.allocate_space(egui::vec2(ui.available_size().x, 0.));
        });
    ui.separator();
    ui.button("Delete").clicked();
}
