use std::ops::RangeInclusive;

use egui::{SidePanel, Slider};

use egui_node_graph::GraphEditorState;

use csc_core::node_model::{ MyNodeData, ImageType, MyValueType, NodeType };
use csc_core::graph_model::MyGraphState;
use csc_core::node_property::NodeProperty;

pub fn build_properties_panel(ctx: &egui::Context, graph_editor_state: &mut GraphEditorState<MyNodeData, ImageType, MyValueType, NodeType, MyGraphState>) {
    SidePanel::right("properties_panel").default_width(450.).show(&ctx, |ui| {
               
        if let Some(node_id) = graph_editor_state.active_node {
            
            let active_node = &mut graph_editor_state.graph.nodes[node_id];
            let node_type = active_node.user_data.node_type;
            if active_node.user_data.node_type == NodeType::Blur {
                ui.label(node_type.name().as_ref());
                ui.separator();
                
                if let NodeProperty::Float(values) = &mut active_node.user_data.node_properties[0] {

                    let range = RangeInclusive::new(*values.min(), *values.max());
                    let name = values.name().clone();
                    ui.add(egui::Slider::new(values.value(), range ).text(name));
                }
            }
        }
     });
}