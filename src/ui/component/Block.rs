use crate::{screen::coordinate::Coordinate2D, ui::view::viewable::Viewable};

pub struct Border {
    left: usize,
    right: usize,
    top: usize,
    bottom: usize,
}

pub struct BlockView {
    width: usize,
    height: usize,
    border: Border,
}

impl BlockView {
    pub fn new(width: usize, height: usize) -> BlockView {
        BlockView {
            width,
            height,
            border: Border {
                left: 1,
                right: 1,
                top: 1,
                bottom: 1,
            },
        }
    }
}

impl Viewable for BlockView {
    fn draw<F: FnMut(crate::screen::coordinate::Coordinate2D, char) -> ()>(&self, mut set_char: F) {
        if self.border.left != 0 {
            for i in 0..self.height {
                set_char(Coordinate2D(i, 0), '|');
            }
        }
        if self.border.top != 0 {
            for i in 0..self.width {
                set_char(Coordinate2D(0, i), '-');
            }
        }
        if self.border.right != 0 {
            for i in 0..self.height {
                set_char(Coordinate2D(i, self.width - 1), '|');
            }
        }
        if self.border.bottom != 0 {
            for i in 0..self.width {
                set_char(Coordinate2D(self.height - 1, i), '-');
            }
        }
    }
}
