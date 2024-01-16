use std::borrow::Cow;

use egui_node_graph::{NodeDataTrait, NodeTemplateIter};

use crate::{
    graph_model::NodeGraphState,
    node_model::{GraphDataType, GraphValueType, MyResponse},
    node_property::NodeProperty,
};

/// The available categories that are shown in the node finder.
#[derive(Clone, Copy, Eq, PartialEq, Hash)]
pub enum NodeCategory {
    IO,
    Filters,
}

impl NodeCategory {
    fn name(&self) -> &'static str {
        match self {
            NodeCategory::IO => "IO",
            NodeCategory::Filters => "Filters",
        }
    }
}

/// The types of nodes.
#[derive(Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "persistence", derive(serde::Serialize, serde::Deserialize))]
pub enum NodeType {
    Blur,
    Read,
    Write,
}

impl NodeType {
    pub fn name(&self) -> Cow<'_, str> {
        match self {
            NodeType::Blur => Cow::Borrowed("Blur"),
            NodeType::Read => Cow::Borrowed("Read"),
            NodeType::Write => Cow::Borrowed("Write"),
        }
    }

    pub fn category_name(&self) -> &'static str {
        match self {
            NodeType::Blur => NodeCategory::name(&NodeCategory::Filters),
            NodeType::Read => NodeCategory::name(&NodeCategory::IO),
            NodeType::Write => NodeCategory::name(&NodeCategory::IO),
        }
    }

    pub fn inputs(&self) -> Vec<GraphDataType> {
        match self {
            NodeType::Blur => vec![GraphDataType::RGB, GraphDataType::Alpha],
            NodeType::Read => vec![],
            NodeType::Write => vec![GraphDataType::RGB, GraphDataType::Alpha],
        }
    }

    pub fn outputs(&self) -> Vec<GraphDataType> {
        match self {
            NodeType::Blur => vec![GraphDataType::RGB, GraphDataType::Alpha],
            NodeType::Read => vec![GraphDataType::RGB, GraphDataType::Alpha],
            NodeType::Write => vec![],
        }
    }
}

/// GraphNodeData holds all the elements that define a node.
/// This way new nodes can be added easily by editing this file.
#[cfg_attr(feature = "persistence", derive(serde::Serialize, serde::Deserialize))]
pub struct GraphNodeData {
    pub node_type: NodeType,
    pub properties: Vec<NodeProperty>,
}

impl GraphNodeData {
    pub fn new(node_type: NodeType) -> Self {
        match node_type {
            NodeType::Blur => GraphNodeData {
                node_type,
                properties: vec![
                    NodeProperty::new_float("Intensity".to_string(), 1.0, 10.0, 1.0, 1.0),
                    NodeProperty::new_choice(
                        "Type".to_string(),
                        vec!["Box".to_string(), "Gaussian".to_string()],
                        0,
                    ),
                ],
            },
            NodeType::Read => GraphNodeData {
                node_type,
                properties: vec![NodeProperty::new_path_list()],
            },
            NodeType::Write => GraphNodeData {
                node_type,
                properties: vec![],
            },
        }
    }
}

// The egui_node_graph crate needs a vector of all NodeType variants.
// This is redundant and should be changed.
pub struct AllNodeTemplates;

impl NodeTemplateIter for AllNodeTemplates {
    type Item = NodeType;

    fn all_kinds(&self) -> Vec<Self::Item> {
        // This function must return a list of node kinds, which the node finder
        // will use to display it to the user. Crates like strum can reduce the
        // boilerplate in enumerating all variants of an enum.
        vec![NodeType::Blur, NodeType::Read, NodeType::Write]
    }
}

// Also needed by egui_node_graph but we are not using it (yet?).
impl NodeDataTrait for GraphNodeData {
    type Response = MyResponse;
    type UserState = NodeGraphState;
    type DataType = GraphDataType;
    type ValueType = GraphValueType;
}
