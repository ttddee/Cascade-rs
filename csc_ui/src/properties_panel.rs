use std::ffi::OsStr;
use std::ops::RangeInclusive;
use std::path::{Path, PathBuf};

use egui::Ui;

use egui_file::FileDialog;
use egui_node_graph::GraphEditorState;

use csc_core::graph_model::NodeGraphState;
use csc_core::node_model::{CsImageType, MyNodeData, MyValueType, NodeType};
use csc_core::node_property::{ChoiceData, NodeProperty, NumberData, PathListData};

use crate::style::COLOR_BG_DARK;

pub struct PropertiesPanel {
    opened_file: Option<PathBuf>,
    open_file_dialog: Option<FileDialog>,
}

impl PropertiesPanel {
    pub fn new() -> Self {
        Self {
            opened_file: Option::None,
            open_file_dialog: Option::None,
        }
    }

    pub fn show(
        &mut self,
        ui: &mut Ui,
        egui_context: &egui::Context,
        graph_state: &mut GraphEditorState<
            MyNodeData,
            CsImageType,
            MyValueType,
            NodeType,
            NodeGraphState,
        >,
    ) {
        egui::Frame::none().inner_margin(0.).show(ui, |ui| {
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
                                self.show_float_property(ui, data);
                            }
                            NodeProperty::Choice(data) => {
                                self.show_choice_property(ui, data);
                            }
                            NodeProperty::PathList(data) => {
                                self.show_path_list_property(ui, data, egui_context);
                            }
                            _ => {}
                        }
                        ui.add_space(2.);
                    }
                });
            }
        });
    }

    fn show_float_property(&self, ui: &mut Ui, data: &mut NumberData<f32>) {
        let range = RangeInclusive::new(*data.min(), *data.max());
        ui.with_layout(egui::Layout::left_to_right(egui::Align::TOP), |ui| {
            ui.add(egui::Label::new(data.name().clone()));
            ui.add(egui::Slider::new(data.value(), range));
        });
    }

    fn show_choice_property(&self, ui: &mut Ui, data: &mut ChoiceData) {
        let length = data.choices().len();
        let choices = data.choices().clone();
        ui.with_layout(egui::Layout::left_to_right(egui::Align::TOP), |ui| {
            ui.add(egui::Label::new(data.name()));
            egui::ComboBox::from_id_source("combobox")
                .show_index(ui, data.index(), length, |i| &choices[i]);
        });
    }

    fn show_path_list_property(
        &mut self,
        ui: &mut Ui,
        data: &mut PathListData,
        context: &egui::Context,
    ) {
        if ui.button("Load").clicked() {
            let filter = Box::new({
                move |path: &Path| -> bool {
                    path.extension() == Some(OsStr::new("png"))
                        || path.extension() == Some(OsStr::new("jpg"))
                }
            });

            let mut dialog =
                FileDialog::open_file(self.opened_file.clone()).show_files_filter(filter);
            dialog.open();
            self.open_file_dialog = Some(dialog);
        }
        if let Some(dialog) = &mut self.open_file_dialog {
            if dialog.show(context).selected() {
                if let Some(file) = dialog.path() {
                    self.opened_file = Some(file.to_path_buf());
                    data.add(self.opened_file.as_ref().unwrap().clone())
                }
            }
        };
        ui.separator();
        egui::Frame::none()
            .fill(COLOR_BG_DARK)
            .inner_margin(5.)
            .show(ui, |ui| {
                let iter = data.list().iter();
                for element in iter {
                    if ui
                        .add(
                            egui::Label::new(
                                element.clone().into_os_string().into_string().unwrap(),
                            )
                            .sense(egui::Sense::click()),
                        )
                        .clicked()
                    { /* … */ }
                }
                // Expand frame horizontally.
                ui.allocate_space(egui::vec2(ui.available_size().x, 0.));
            });
        ui.separator();
        ui.button("Delete").clicked();
    }
}

impl Default for PropertiesPanel {
    fn default() -> Self {
        Self::new()
    }
}
