pub mod viewable;
use viewable::Viewable;

pub struct View {}

impl Viewable for View {
    fn draw<F: FnMut(crate::screen::coordinate::Coordinate2D, char) -> ()>(&self, mut set_char: F) {
        
    }
}
