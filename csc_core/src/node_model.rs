use std::vec;

use std::borrow::Cow;

use egui::DragValue;

use egui_node_graph::{ NodeId, NodeTemplateIter, NodeTemplateTrait, Graph, 
    InputParamKind, NodeDataTrait, NodeResponse, UserResponseTrait, WidgetValueTrait, DataTypeTrait };

use crate::graph_model::MyGraphState;

type MyGraph = Graph<MyNodeData, MyDataType, MyValueType>;

pub enum NodeCategory {
    IO,
    Filters,
}

pub trait CategoryTrait {
    fn name(&self) -> String;
}

impl CategoryTrait for NodeCategory {
    fn name(&self) -> String {
        match self {
            NodeCategory::IO => String::from("IO"),
            NodeCategory::Filters => String::from("Filters"),
            _ => panic!("Node category does not exist."),
        } 
    }
}

#[derive(Clone, Copy)]
pub enum NodeType {
    Blur,
    Read,
    Write,
}

pub trait NodeTypeTrait {
    fn name(&self) -> String;
}

impl NodeTypeTrait for NodeType {
    fn name(&self) -> String {
        match self {
            NodeType::Blur => String::from("Blur"),
            NodeType::Read => String::from("Read"),
            NodeType::Write => String::from("Write"),
            _ => panic!("Node type does not exist."),
        } 
    }
}


//--------------------------------------------


pub enum ImageType {
    RGB,
    Alpha,
}

pub enum NodeProperty {
    Float (Vec<f32>),
    Int (Vec<i32>),
}

pub struct NodeModel {
    node_type: NodeType,
    category: NodeCategory,
    inputs: Vec<ImageType>,
    outputs: Vec<ImageType>,
    properties: Vec<NodeProperty>,
}

// ------------------------------- MyNodeTemplate

/// NodeTemplate is a mechanism to define node templates. It's what the graph
/// will display in the "new node" popup. The user code needs to tell the
/// library how to convert a NodeTemplate into a Node.
#[derive(Clone, Copy)]
#[cfg_attr(feature = "persistence", derive(serde::Serialize, serde::Deserialize))]
pub enum MyNodeTemplate {
    Blur,
    Read,
    Write,
}

// A trait for the node kinds, which tells the library how to build new nodes
// from the templates in the node finder
impl NodeTemplateTrait for MyNodeTemplate{
    type NodeData = MyNodeData;
    type DataType = MyDataType;
    type ValueType = MyValueType;
    type UserState = MyGraphState;
    type CategoryType = &'static str;

    fn node_finder_label(&self, _user_state: &mut Self::UserState) -> Cow<'_, str> {
        Cow::Borrowed(match self {
            MyNodeTemplate::Blur => "Blur",
            MyNodeTemplate::Read => "Read",
            MyNodeTemplate::Write => "Write",
        })
    }

    // this is what allows the library to show collapsible lists in the node finder.
    fn node_finder_categories(&self, _user_state: &mut Self::UserState) -> Vec<&'static str> {
        match self {
            MyNodeTemplate::Blur => vec!["Filters"],
            MyNodeTemplate::Read 
            | MyNodeTemplate::Write => vec!["IO"],
        }
    }

    fn node_graph_label(&self, user_state: &mut Self::UserState) -> String {
        // It's okay to delegate this to node_finder_label if you don't want to
        // show different names in the node finder and the node itself.
        self.node_finder_label(user_state).into()
    }

    fn user_data(&self, _user_state: &mut Self::UserState) -> Self::NodeData {
        MyNodeData { template: *self }
    }

    fn build_node(
        &self,
        graph: &mut Graph<Self::NodeData, Self::DataType, Self::ValueType>,
        _user_state: &mut Self::UserState,
        node_id: NodeId,
    ) {
        // The nodes are created empty by default. This function needs to take
        // care of creating the desired inputs and outputs based on the template

        // We define some closures here to avoid boilerplate. Note that this is
        // entirely optional.
        let input_rgb = |graph: &mut MyGraph, name: &str| {
            graph.add_input_param(
                node_id,
                name.to_string(),
                MyDataType::RGB,
                MyValueType::RGB { value: 0.0 },
                InputParamKind::ConnectionOnly,
                true,
            );
        };
        let input_alpha = |graph: &mut MyGraph, name: &str| {
            graph.add_input_param(
                node_id,
                name.to_string(),
                MyDataType::Alpha,
                MyValueType::Alpha { value: 0.0 },
                InputParamKind::ConnectionOnly,
                true,
            );
        };

        let output_rgb = |graph: &mut MyGraph, name: &str| {
            graph.add_output_param(node_id, name.to_string(), MyDataType::RGB);
        };
        let output_alpha = |graph: &mut MyGraph, name: &str| {
            graph.add_output_param(node_id, name.to_string(), MyDataType::Alpha);
        };
            


        match self {
            MyNodeTemplate::Blur => {
                input_rgb(graph, "RGB");
                input_alpha(graph, "Alpha");
                output_rgb(graph, "RGB");
                output_alpha(graph, "Alpha");
            }
            MyNodeTemplate::Read => {
                output_rgb(graph, "RGB");
                output_alpha(graph, "Alpha");
            }
            MyNodeTemplate::Write => {
                input_rgb(graph, "RGB");
                input_alpha(graph, "Alpha");
            }
        }
    }
}


// ------------------------------- MyDataType

/// `DataType`s are what defines the possible range of connections when
/// attaching two ports together. The graph UI will make sure to not allow
/// attaching incompatible datatypes.
#[derive(PartialEq, Eq)]
#[cfg_attr(feature = "persistence", derive(serde::Serialize, serde::Deserialize))]
pub enum MyDataType {
    RGB,
    Alpha,
}

// A trait for the data types, to tell the library how to display them
impl DataTypeTrait<MyGraphState> for MyDataType {
    fn data_type_color(&self, _user_state: &mut MyGraphState) -> ecolor::Color32 {
        match self {
            MyDataType::RGB => egui::Color32::from_rgb(229, 70, 61),
            MyDataType::Alpha => egui::Color32::from_rgb(35, 114, 239),
        }
    }

    fn name(&self) -> Cow<'_, str> {
        match self {
            MyDataType::RGB => Cow::Borrowed("RGB"),
            MyDataType::Alpha => Cow::Borrowed("Alpha"),
        }
    }
}

// ------------------------------- MyNodeData

/// The NodeData holds a custom data struct inside each node. It's useful to
/// store additional information that doesn't live in parameters. For this
/// example, the node data stores the template (i.e. the "type") of the node.
#[cfg_attr(feature = "persistence", derive(serde::Serialize, serde::Deserialize))]
pub struct MyNodeData {
    template: MyNodeTemplate,
}
impl NodeDataTrait for MyNodeData {
    type Response = MyResponse;
    type UserState = MyGraphState;
    type DataType = MyDataType;
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
        _graph: &Graph<MyNodeData, MyDataType, MyValueType>,
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
        let is_active = user_state
            .active_node
            .map(|id| id == node_id)
            .unwrap_or(false);

        // Pressing the button will emit a custom user response to either set,
        // or clear the active node. These responses do nothing by themselves,
        // the library only makes the responses available to you after the graph
        // has been drawn. See below at the update method for an example.
        if !is_active {
            if ui.button("👁 Set active").clicked() {
                responses.push(NodeResponse::User(MyResponse::SetActiveNode(node_id)));
            }
        } else {
            let button =
                egui::Button::new(egui::RichText::new("👁 Active").color(egui::Color32::BLACK))
                    .fill(egui::Color32::GOLD);
            if ui.add(button).clicked() {
                responses.push(NodeResponse::User(MyResponse::ClearActiveNode));
            }
        }

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

impl MyValueType {
    // Tries to downcast this value type to a vector
    // pub fn try_to_vec2(self) -> anyhow::Result<egui::Vec2> {
    //     if let MyValueType::Vec2 { value } = self {
    //         Ok(value)
    //     } else {
    //         anyhow::bail!("Invalid cast from {:?} to vec2", self)
    //     }
    // }

    // /// Tries to downcast this value type to a scalar
    // pub fn try_to_scalar(self) -> anyhow::Result<f32> {
    //     if let MyValueType::Scalar { value } = self {
    //         Ok(value)
    //     } else {
    //         anyhow::bail!("Invalid cast from {:?} to scalar", self)
    //     }
    // }
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
    SetActiveNode(NodeId),
    ClearActiveNode,
}

impl UserResponseTrait for MyResponse {}

// ------------------------------- AllMyNodeTemplates

pub struct AllMyNodeTemplates;
impl NodeTemplateIter for AllMyNodeTemplates {
    type Item = MyNodeTemplate;

    fn all_kinds(&self) -> Vec<Self::Item> {
        // This function must return a list of node kinds, which the node finder
        // will use to display it to the user. Crates like strum can reduce the
        // boilerplate in enumerating all variants of an enum.
        vec![
            MyNodeTemplate::Blur,
            MyNodeTemplate::Read,
            MyNodeTemplate::Write,
        ]
    }
}