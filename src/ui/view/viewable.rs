use crate::screen::coordinate::Coordinate2D;

pub trait Viewable {
    fn draw<F: FnMut(Coordinate2D, char) -> ()>(&self, set_char: F);
}
