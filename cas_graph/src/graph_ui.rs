use egui::{Color32, Rect, Sense, Stroke, StrokeKind, Ui, Vec2};

use crate::{node_data::NodeData, node_graph::NodeGraph};

const NODE_SIZE: Vec2 = Vec2::new(100., 50.);
const CORNER_RADIUS: f32 = 5.;

pub struct GraphUI {}

impl GraphUI {
    pub fn draw_graph(&self, ui: &mut Ui, graph: &NodeGraph) {
        let nodes = graph.nodes();
        for node in nodes {
            Self::draw_node(ui, node);
        }
    }

    fn draw_node(ui: &mut Ui, data: &NodeData) {
        let rect = Rect::from_min_size(data.position(), NODE_SIZE);
        let response = ui.allocate_rect(rect, Sense::click().union(Sense::drag()));

        if response.clicked() {
            dbg!("Click started");
        } else if response.drag_started() {
            dbg!("Drag started");
            dbg!(response.drag_delta());
        } else if response.drag_stopped() {
            dbg!("Drag stopped");
        }

        ui.painter().rect(
            rect,
            CORNER_RADIUS,
            Color32::WHITE,
            Stroke::NONE,
            StrokeKind::Middle,
        );
    }
}
