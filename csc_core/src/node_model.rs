use std::vec;

pub enum NodeType {
    Blur,
    Read,
    Write,
}

pub enum NodeCategory {
    IO,
    Filters,
}

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