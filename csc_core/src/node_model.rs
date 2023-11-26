use std::vec;

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
    Int (Vec<f32>),
}

pub struct NodeModel {
    node_type: NodeType,
    category: NodeCategory,
    inputs: Vec<ImageType>,
    outputs: Vec<ImageType>,
    properties: Vec<NodeProperty>,
}