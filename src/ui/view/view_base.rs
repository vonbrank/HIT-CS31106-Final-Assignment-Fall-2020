use crate::renderer::{canvas::GLOBAL_CANVAS, coordinate::Coordinate2D};

pub trait ViewBase {
    fn draw(&self) {
        let global_canvas = GLOBAL_CANVAS.lock().unwrap();
        let mut global_canvas = global_canvas.borrow_mut();

        let width = 60;
        let height = 20;
        global_canvas.resize(width, height);

        for i in 1..width {
            global_canvas.set_char_in_matrix(Coordinate2D(0, i - 1), '-');
            global_canvas.set_char_in_matrix(Coordinate2D(height - 1, i - 1), '-');
        }
        for i in 1..height {
            global_canvas.set_char_in_matrix(Coordinate2D(i - 1, 0), '|');
            global_canvas.set_char_in_matrix(Coordinate2D(i - 1, width - 1), '|');
        }
        global_canvas.set_char_in_matrix(Coordinate2D(0, 0), '+');
        global_canvas.set_char_in_matrix(Coordinate2D(height - 1, 0), '+');
        global_canvas.set_char_in_matrix(Coordinate2D(0, width - 1), '+');
        global_canvas.set_char_in_matrix(Coordinate2D(height - 1, width - 1), '+');
    }
}
