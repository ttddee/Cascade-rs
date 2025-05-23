use egui::Pos2;
use petgraph::stable_graph::StableGraph;

use crate::node_data::NodeData;

type Graph = StableGraph<NodeData, ()>;

pub struct NodeGraph {
    graph: Graph,
}

impl NodeGraph {
    pub fn new() -> Self {
        Self {
            graph: Graph::new(),
        }
    }

    pub fn graph(&self) -> &Graph {
        &self.graph
    }

    pub fn nodes(&self) -> Vec<&NodeData> {
        self.graph.node_weights().collect::<Vec<&NodeData>>()
    }

    pub fn add_node(&mut self, position: Pos2) {
        let node = NodeData::new(position);
        self.graph.add_node(node);
    }
}
