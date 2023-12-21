use std::vec;

use std::borrow::Cow;

use egui_node_graph::{
    DataTypeTrait, Graph, InputParamKind, NodeDataTrait, NodeId, NodeResponse, NodeTemplateIter,
    NodeTemplateTrait, UserResponseTrait, WidgetValueTrait,
};

use crate::graph_model::MyGraphState;

use crate::node_property::NodeProperty;

type MyGraph = Graph<MyNodeData, ImageType, MyValueType>;

//--------------------------------------------

pub struct NodeModel {
    node_type: NodeType,
    category: NodeCategory,
    inputs: Vec<ImageType>,
    outputs: Vec<ImageType>,
    properties: Vec<NodeProperty>,
}

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

// ------------------------------- MyDataType

/// `DataType`s are what defines the possible range of connections when
/// attaching two ports together. The graph UI will make sure to not allow
/// attaching incompatible datatypes.
#[derive(PartialEq, Eq)]
#[cfg_attr(feature = "persistence", derive(serde::Serialize, serde::Deserialize))]
pub enum ImageType {
    RGB,
    Alpha,
}

// A trait for the data types, to tell the library how to display them
impl DataTypeTrait<MyGraphState> for ImageType {
    fn data_type_color(&self, _user_state: &mut MyGraphState) -> ecolor::Color32 {
        match self {
            ImageType::RGB => ecolor::Color32::from_rgb(229, 70, 61),
            ImageType::Alpha => ecolor::Color32::from_rgb(35, 114, 239),
        }
    }

    fn name(&self) -> Cow<'_, str> {
        match self {
            ImageType::RGB => Cow::Borrowed("RGB"),
            ImageType::Alpha => Cow::Borrowed("Alpha"),
        }
    }
}

// ------------------------------- NodeType

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

    fn category_name(&self) -> &'static str {
        match self {
            NodeType::Blur => NodeCategory::name(&NodeCategory::Filters),
            NodeType::Read => NodeCategory::name(&NodeCategory::IO),
            NodeType::Write => NodeCategory::name(&NodeCategory::IO),
        }
    }

    fn inputs(&self) -> Vec<ImageType> {
        match self {
            NodeType::Blur => vec![ImageType::RGB, ImageType::Alpha],
            NodeType::Read => vec![],
            NodeType::Write => vec![ImageType::RGB, ImageType::Alpha],
        }
    }

    fn outputs(&self) -> Vec<ImageType> {
        match self {
            NodeType::Blur => vec![ImageType::RGB, ImageType::Alpha],
            NodeType::Read => vec![ImageType::RGB, ImageType::Alpha],
            NodeType::Write => vec![],
        }
    }
}

/// NodeTemplate is a mechanism to define node templates. It's what the graph
/// will display in the "new node" popup. The user code needs to tell the
/// library how to convert a NodeTemplate into a Node.

// A trait for the node kinds, which tells the library how to build new nodes
// from the templates in the node finder
impl NodeTemplateTrait for NodeType {
    type NodeData = MyNodeData;
    type DataType = ImageType;
    type ValueType = MyValueType;
    type UserState = MyGraphState;
    type CategoryType = &'static str;

    fn node_finder_label(&self, _user_state: &mut Self::UserState) -> Cow<'_, str> {
        self.name()
    }

    // this is what allows the library to show collapsible lists in the node finder.
    fn node_finder_categories(&self, _user_state: &mut Self::UserState) -> Vec<&'static str> {
        vec![self.category_name()]
    }

    fn node_graph_label(&self, user_state: &mut Self::UserState) -> String {
        // It's okay to delegate this to node_finder_label if you don't want to
        // show different names in the node finder and the node itself.
        self.node_finder_label(user_state).into()
    }

    fn node_data(&self, _user_state: &mut Self::UserState) -> Self::NodeData {
        let properties: Vec<NodeProperty> = match self {
            NodeType::Blur => vec![
                NodeProperty::new_float("Intensity".to_string(), 1.0, 10.0, 1.0, 1.0),
                NodeProperty::new_choice(
                    "Type".to_string(),
                    vec!["Box".to_string(), "Gaussian".to_string()],
                    0,
                ),
            ],
            NodeType::Read => vec![NodeProperty::new_float(
                "Intensity".to_string(),
                1.0,
                10.0,
                1.0,
                1.0,
            )],
            NodeType::Write => vec![NodeProperty::new_float(
                "Intensity".to_string(),
                1.0,
                10.0,
                1.0,
                1.0,
            )],
        };
        MyNodeData {
            node_type: *self,
            node_properties: properties,
        }
    }

    fn build_node(
        &self,
        graph: &mut Graph<Self::NodeData, Self::DataType, Self::ValueType>,
        _user_state: &mut Self::UserState,
        node_id: NodeId,
    ) {
        // The nodes are created empty by default. This function needs to take
        // care of creating the desired inputs and outputs based on the template

        let input_rgb = |graph: &mut MyGraph, name: &str| {
            graph.add_input_param(
                node_id,
                name.to_string(),
                ImageType::RGB,
                MyValueType::RGB { value: 0.0 },
                InputParamKind::ConnectionOnly,
                true,
            );
        };
        let input_alpha = |graph: &mut MyGraph, name: &str| {
            graph.add_input_param(
                node_id,
                name.to_string(),
                ImageType::Alpha,
                MyValueType::Alpha { value: 0.0 },
                InputParamKind::ConnectionOnly,
                true,
            );
        };

        let output_rgb = |graph: &mut MyGraph, name: &str| {
            graph.add_output_param(node_id, name.to_string(), ImageType::RGB);
        };
        let output_alpha = |graph: &mut MyGraph, name: &str| {
            graph.add_output_param(node_id, name.to_string(), ImageType::Alpha);
        };

        let inputs = self.inputs();
        let iter = inputs.iter();
        for input in iter {
            match input {
                ImageType::RGB => input_rgb(graph, "RGB"),
                ImageType::Alpha => input_alpha(graph, "Alpha"),
            }
        }

        let outputs = self.outputs();
        let iter = outputs.iter();
        for output in iter {
            match output {
                ImageType::RGB => output_rgb(graph, "RGB"),
                ImageType::Alpha => output_alpha(graph, "Alpha"),
            }
        }
    }
}

// ------------------------------- MyNodeData

/// The NodeData holds a custom data struct inside each node. It's useful to
/// store additional information that doesn't live in parameters. For this
/// example, the node data stores the template (i.e. the "type") of the node.
#[cfg_attr(feature = "persistence", derive(serde::Serialize, serde::Deserialize))]
pub struct MyNodeData {
    pub node_type: NodeType,
    pub node_properties: Vec<NodeProperty>,
}
impl NodeDataTrait for MyNodeData {
    type Response = MyResponse;
    type UserState = MyGraphState;
    type DataType = ImageType;
    type ValueType = MyValueType;

    // This method will be called when drawing each node. This allows adding
    // extra ui elements inside the nodes. In this case, we create an "active"
    // button which introduces the concept of having an active node in the
    // graph. This is done entirely from user code with no modifications to the
    // node graph library.
    fn bottom_ui(
        &self,
        ui: &mut egui::Ui,
        node_id: NodeId,
        _graph: &Graph<MyNodeData, ImageType, MyValueType>,
        user_state: &mut Self::UserState,
    ) -> Vec<NodeResponse<MyResponse, MyNodeData>>
    where
        MyResponse: UserResponseTrait,
    {
        // This logic is entirely up to the user. In this case, we check if the
        // current node we're drawing is the active one, by comparing against
        // the value stored in the global user state, and draw different button
        // UIs based on that.

        let mut responses = vec![];
        // let is_active = user_state
        //     .active_node
        //     .map(|id| id == node_id)
        //     .unwrap_or(false);

        // Pressing the button will emit a custom user response to either set,
        // or clear the active node. These responses do nothing by themselves,
        // the library only makes the responses available to you after the graph
        // has been drawn. See below at the update method for an example.
        // if !is_active {
        //     if ui.button("👁 Set active").clicked() {
        //         responses.push(NodeResponse::User(MyResponse::SetActiveNode(node_id)));
        //     }
        // } else {
        //     let button =
        //         egui::Button::new(egui::RichText::new("👁 Active").color(egui::Color32::BLACK))
        //             .fill(egui::Color32::GOLD);
        //     if ui.add(button).clicked() {
        //         responses.push(NodeResponse::User(MyResponse::ClearActiveNode));
        //     }
        // }

        responses
    }
}

// ------------------------------- MyValueType

/// In the graph, input parameters can optionally have a constant value. This
/// value can be directly edited in a widget inside the node itself.
///
/// There will usually be a correspondence between DataTypes and ValueTypes. But
/// this library makes no attempt to check this consistency. For instance, it is
/// up to the user code in this example to make sure no parameter is created
/// with a DataType of Scalar and a ValueType of Vec2.
#[derive(Copy, Clone, Debug)]
#[cfg_attr(feature = "persistence", derive(serde::Serialize, serde::Deserialize))]
pub enum MyValueType {
    RGB { value: f32 },
    Alpha { value: f32 },
}

impl Default for MyValueType {
    fn default() -> Self {
        // NOTE: This is just a dummy `Default` implementation. The library
        // requires it to circumvent some internal borrow checker issues.
        Self::RGB { value: 0.0 }
    }
}

impl WidgetValueTrait for MyValueType {
    type Response = MyResponse;
    type UserState = MyGraphState;
    type NodeData = MyNodeData;
    fn value_widget(
        &mut self,
        param_name: &str,
        _node_id: NodeId,
        ui: &mut egui::Ui,
        _user_state: &mut MyGraphState,
        _node_data: &MyNodeData,
    ) -> Vec<MyResponse> {
        // This trait is used to tell the library which UI to display for the
        // inline parameter widgets.
        match self {
            MyValueType::RGB { value } => {
                ui.horizontal(|ui| {
                    ui.label(param_name);
                });
            }
            MyValueType::Alpha { value } => {
                ui.horizontal(|ui| {
                    ui.label(param_name);
                });
            }
        }
        // This allows you to return your responses from the inline widgets.
        Vec::new()
    }
}

// ------------------------------- MyResponse

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

// ------------------------------- AllMyNodeTemplates

pub struct AllMyNodeTemplates;
impl NodeTemplateIter for AllMyNodeTemplates {
    type Item = NodeType;

    fn all_kinds(&self) -> Vec<Self::Item> {
        // This function must return a list of node kinds, which the node finder
        // will use to display it to the user. Crates like strum can reduce the
        // boilerplate in enumerating all variants of an enum.
        vec![NodeType::Blur, NodeType::Read, NodeType::Write]
    }
}
