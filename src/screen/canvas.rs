use std::{cell::RefCell, sync::Mutex};

use lazy_static::lazy_static;

use crate::ui::view::viewable::Viewable;

use super::coordinate::Coordinate2D;

pub struct Canvas {
    width: usize,
    height: usize,
    pub char_matrix: Vec<Vec<char>>,
}

impl Canvas {
    pub fn new(width: usize, height: usize) -> Canvas {
        let char_matrix: Vec<Vec<char>> = vec![vec![' '; width]; height];
        Canvas {
            width,
            height,
            char_matrix,
        }
    }

    pub fn resize(&mut self, width: usize, height: usize) {
        let char_matrix: Vec<Vec<char>> = vec![vec![' '; width]; height];
        self.width = width;
        self.height = height;
        self.char_matrix = char_matrix;
    }

    pub fn set_char_in_matrix(&mut self, coordinate: Coordinate2D, value: char) {
        let Coordinate2D(x, y) = coordinate;
        if x >= self.height || y >= self.width {
            return;
        }
        self.char_matrix.get_mut(x).unwrap()[y] = value;
    }

    pub fn render_view<T: Viewable>(&mut self, view_able: &mut T) {
        view_able.draw(|coordinate, value| {
            println!("current coordinate: ({}, {})", coordinate.0, coordinate.1);
            self.set_char_in_matrix(coordinate, value)
        });
    }

    pub fn draw_on_screen(&self) {
        self.char_matrix.iter().for_each(|line| {
            line.iter().for_each(|c| {
                print!("{}", c);
            });
            println!();
        })
    }
}

// lazy_static! {
//     pub static ref GLOBAL_CANVAS: Mutex<RefCell<Canvas>> =
//         Mutex::new(RefCell::new(Canvas::new(60, 20)));
// }