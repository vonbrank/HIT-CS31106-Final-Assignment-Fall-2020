use std::cmp::min;

use super::viewnode::{self, BoxModelAttribute, ViewNode, ViewNodeType};

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

    pub fn draw_on_screen(&self) {
        self.char_matrix.iter().for_each(|line| {
            line.iter().for_each(|c| {
                print!("{}", c);
            });
            println!();
        })
    }

    pub fn render_view_node_tree(&mut self, root: &RootViewNode) {
        self.render_node_with_constraint(
            &root.0,
            &RenderConstraint(Coordinate2D(0, 0), Coordinate2D(self.height, self.width)),
        )
    }

    fn render_node_with_constraint(&mut self, view_node: &ViewNode, constraint: &RenderConstraint) {
        let RenderConstraint(
            Coordinate2D(left_top_x, left_top_y),
            Coordinate2D(bottom_right_x, bottom_right_y),
        ) = constraint;

        match &view_node.node_type {
            ViewNodeType::LayoutType(layout_type) => {
                let inner_constraint =
                    Self::calculate_inner_constraint(&constraint, &view_node.box_model_attribute);

                println!("layout type, inner constraint = {:?}", inner_constraint);

                if inner_constraint.1 .0 > inner_constraint.0 .0
                    && inner_constraint.1 .1 > inner_constraint.0 .1
                {
                    println!("render child node");
                    view_node.child_nodes.iter().for_each(|childe_node| {
                        println!("render text node");
                        self.render_node_with_constraint(childe_node, &inner_constraint);
                    });
                }
            }
            ViewNodeType::ContentType(content_type) => match content_type {
                viewnode::ContentType::Text(text) => {
                    println!(
                        "content type, constraint = {:?}, text = {}",
                        constraint, text
                    );
                    for (index, c) in text.chars().enumerate() {
                        self.char_matrix[*left_top_x][left_top_y + index] = c;
                    }
                }
            },
        };

        let mut node_width = 0;
        let mut node_height = 0;
        match &view_node.box_model_attribute.width {
            Some(width) => {
                node_width = *width;
            }
            None => {}
        }
        match &view_node.box_model_attribute.height {
            Some(height) => {
                node_height = *height;
            }
            None => {}
        }
        node_width = min(node_width, bottom_right_y - left_top_y);
        node_height = min(node_width, bottom_right_x - left_top_x);

        if node_width == 0 || node_height == 0 || view_node.box_model_attribute.border.is_none() {
            return;
        }

        for i in 0..node_width {
            self.char_matrix[0][i] = '-';
            self.char_matrix[node_height - 1][i] = '-';
        }

        for i in 0..node_height {
            self.char_matrix[i][0] = '|';
            self.char_matrix[i][node_width - 1] = '|';
        }
    }

    fn calculate_inner_constraint(
        outer_constraint: &RenderConstraint,
        box_model_attribute: &BoxModelAttribute,
    ) -> RenderConstraint {
        let mut border_left: u32 = 0;
        let mut border_top: u32 = 0;
        let mut border_right: u32 = 0;
        let mut border_bottom: u32 = 0;

        match &box_model_attribute.border {
            Some(border) => {
                border_left = border.left as u32;
                border_top = border.top as u32;
                border_right = border.right as u32;
                border_bottom = border.bottom as u32;
            }
            _ => {}
        };

        let mut padding_left: u32 = 0;
        let mut padding_top: u32 = 0;
        let mut padding_right: u32 = 0;
        let mut padding_bottom: u32 = 0;

        match &box_model_attribute.padding {
            Some(padding) => {
                padding_left = padding.left as u32;
                padding_top = padding.top as u32;
                padding_right = padding.right as u32;
                padding_bottom = padding.bottom as u32;
            }
            _ => {}
        };

        let inner_left_top_x = outer_constraint.0 .0 as u32 + border_top + padding_top;
        let inner_left_top_y = outer_constraint.0 .1 as u32 + border_left + padding_left;
        let inner_bottom_right_x = outer_constraint.1 .0 as u32 - border_bottom - padding_bottom;
        let inner_bottom_right_y = outer_constraint.1 .1 as u32 - border_right - padding_right;

        RenderConstraint(
            Coordinate2D(
                if inner_left_top_x > 0 {
                    inner_left_top_x as usize
                } else {
                    0
                },
                if inner_left_top_y > 0 {
                    inner_left_top_y as usize
                } else {
                    0
                },
            ),
            Coordinate2D(
                if inner_bottom_right_x > 0 {
                    inner_bottom_right_x as usize
                } else {
                    0
                },
                if inner_bottom_right_y > 0 {
                    inner_bottom_right_y as usize
                } else {
                    0
                },
            ),
        )
    }
}

pub struct RootViewNode(pub ViewNode);

#[derive(Debug)]
pub struct Coordinate2D(pub usize, pub usize);

#[derive(Debug)]
struct RenderConstraint(Coordinate2D, Coordinate2D);
