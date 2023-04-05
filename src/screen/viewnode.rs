use std::rc::Rc;

pub struct ViewNode {
    pub box_model_attribute: BoxModelAttribute,
    pub node_type: ViewNodeType,

    pub child_nodes: Vec<Rc<ViewNode>>,
}

pub struct BoxModelAttribute {
    pub padding: Option<Padding>,
    pub margin: Option<Margin>,
    pub border: Option<Border>,
    pub width: Option<usize>,
    pub height: Option<usize>,
}

pub struct BoxModelQuadruple {
    pub left: usize,
    pub top: usize,
    pub right: usize,
    pub bottom: usize,
}

pub type Border = BoxModelQuadruple;
pub type Margin = BoxModelQuadruple;
pub type Padding = BoxModelQuadruple;

pub enum ViewNodeType {
    LayoutType(LayoutType),
    ContentType(ContentType),
}

pub enum LayoutType {
    Column,
    Row,
    Box,
}

pub enum ContentType {
    Text(String),
}
