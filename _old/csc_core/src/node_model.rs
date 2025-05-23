use std::vec;

use std::borrow::Cow;

use egui_node_graph::{
    DataTypeTrait, Graph, InputParamKind, NodeId, NodeTemplateTrait, UserResponseTrait,
    WidgetValueTrait,
};

use crate::graph_model::NodeGraphState;

use crate::node_data::{GraphNodeData, NodeType};

type MyGraph = Graph<GraphNodeData, GraphDataType, GraphValueType>;

/// `DataType`s are what defines the possible range of connections when
/// attaching two ports together. The graph UI will make sure to not allow
/// attaching incompatible datatypes.
#[derive(PartialEq, Eq)]
pub enum GraphDataType {
    RGB,
    Alpha,
}

/// A trait for the data types, to tell the library how to display them
impl DataTypeTrait<NodeGraphState> for GraphDataType {
    fn data_type_color(&self, _user_state: &NodeGraphState) -> ecolor::Color32 {
        match self {
            GraphDataType::RGB => ecolor::Color32::from_rgb(229, 70, 61),
            GraphDataType::Alpha => ecolor::Color32::from_rgb(35, 114, 239),
        }
    }

    fn name(&self) -> Cow<'_, str> {
        match self {
            GraphDataType::RGB => Cow::Borrowed("RGB"),
            GraphDataType::Alpha => Cow::Borrowed("Alpha"),
        }
    }
}

/// A trait for the node kinds, which tells the library how to build new nodes
/// from the templates in the node finder.
impl NodeTemplateTrait for NodeType {
    type NodeData = GraphNodeData;
    type DataType = GraphDataType;
    type ValueType = GraphValueType;
    type UserState = NodeGraphState;
    type CategoryType = &'static str;

    fn node_finder_label(&self, _user_state: &Self::UserState) -> Cow<'_, str> {
        self.name()
    }

    // this is what allows the library to show collapsible lists in the node finder.
    fn node_finder_categories(&self, _user_state: &Self::UserState) -> Vec<&'static str> {
        vec![self.category_name()]
    }

    fn node_graph_label(&self, user_state: &Self::UserState) -> String {
        // It's okay to delegate this to node_finder_label if you don't want to
        // show different names in the node finder and the node itself.
        self.node_finder_label(user_state).into()
    }

    fn node_data(&self, _user_state: &Self::UserState) -> Self::NodeData {
        GraphNodeData::new(*self)
    }

    fn build_node(
        &self,
        graph: &mut Graph<Self::NodeData, Self::DataType, Self::ValueType>,
        _user_state: &Self::UserState,
        node_id: NodeId,
    ) {
        // The nodes are created empty by default. This function needs to take
        // care of creating the desired inputs and outputs based on the template

        let input_rgb = |graph: &mut MyGraph, name: &str| {
            graph.add_input_param(
                node_id,
                name.to_string(),
                GraphDataType::RGB,
                GraphValueType::RGB,
                InputParamKind::ConnectionOnly,
                true,
            );
        };
        let input_alpha = |graph: &mut MyGraph, name: &str| {
            graph.add_input_param(
                node_id,
                name.to_string(),
                GraphDataType::Alpha,
                GraphValueType::Alpha,
                InputParamKind::ConnectionOnly,
                true,
            );
        };

        let output_rgb = |graph: &mut MyGraph, name: &str| {
            graph.add_output_param(node_id, name.to_string(), GraphDataType::RGB);
        };
        let output_alpha = |graph: &mut MyGraph, name: &str| {
            graph.add_output_param(node_id, name.to_string(), GraphDataType::Alpha);
        };

        let inputs = self.inputs();
        let iter = inputs.iter();
        for input in iter {
            match input {
                GraphDataType::RGB => input_rgb(graph, "RGB"),
                GraphDataType::Alpha => input_alpha(graph, "Alpha"),
            }
        }

        let outputs = self.outputs();
        let iter = outputs.iter();
        for output in iter {
            match output {
                GraphDataType::RGB => output_rgb(graph, "RGB"),
                GraphDataType::Alpha => output_alpha(graph, "Alpha"),
            }
        }
    }
}

/// In the graph, input parameters can optionally have a constant value. This
/// value can be directly edited in a widget inside the node itself.
///
/// There will usually be a correspondence between DataTypes and ValueTypes. But
/// this library makes no attempt to check this consistency. For instance, it is
/// up to the user code in this example to make sure no parameter is created
/// with a DataType of Scalar and a ValueType of Vec2.
#[derive(Copy, Clone, Debug)]
#[cfg_attr(feature = "persistence", derive(serde::Serialize, serde::Deserialize))]
pub enum GraphValueType {
    RGB,
    Alpha,
}

impl Default for GraphValueType {
    fn default() -> Self {
        // NOTE: This is just a dummy `Default` implementation. The library
        // requires it to circumvent some internal borrow checker issues.
        Self::RGB
    }
}

impl WidgetValueTrait for GraphValueType {
    type Response = MyResponse;
    type UserState = NodeGraphState;
    type NodeData = GraphNodeData;
    fn value_widget(
        &mut self,
        param_name: &str,
        _node_id: NodeId,
        ui: &mut egui::Ui,
        _user_state: &NodeGraphState,
        _node_data: &GraphNodeData,
    ) -> Vec<MyResponse> {
        // This trait is used to tell the library which UI to display for the
        // inline parameter widgets.
        match self {
            GraphValueType::RGB {} => {
                ui.horizontal(|ui| {
                    ui.label(param_name);
                });
            }
            GraphValueType::Alpha {} => {
                ui.horizontal(|ui| {
                    ui.label(param_name);
                });
            }
        }
        // This allows you to return your responses from the inline widgets.
        Vec::new()
    }
}

/// The response type is used to encode side-effects produced when drawing a
/// node in the graph. Most side-effects (creating new nodes, deleting existing
/// nodes, handling connections...) are already handled by the library, but this
/// mechanism allows creating additional side effects from user code.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MyResponse {
    //SetActiveNode(NodeId),
    //ClearActiveNode,
}

impl UserResponseTrait for MyResponse {}
