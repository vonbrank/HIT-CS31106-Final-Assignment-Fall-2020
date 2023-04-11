use crate::screen::viewnode::ViewNode;

pub mod divider;
pub mod text;

pub trait Component {
    fn to_view_mut(&mut self) -> ViewNode;
}
