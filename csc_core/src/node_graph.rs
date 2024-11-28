use egui_node_graph::GraphEditorState;

use crate::{
    graph_model::NodeGraphState,
    node_data::{GraphNodeData, NodeType},
    node_model::{GraphDataType, GraphValueType},
};

pub struct NodeGraph {
    pub graph_state:
        GraphEditorState<GraphNodeData, GraphDataType, GraphValueType, NodeType, NodeGraphState>,
    pub user_state: NodeGraphState,
}
