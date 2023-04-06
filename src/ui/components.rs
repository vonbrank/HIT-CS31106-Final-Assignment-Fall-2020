use crate::screen::viewnode::ViewNode;

pub mod text;
pub mod divider;

pub trait Component {
    fn to_view_node(&self) -> ViewNode;
}
