use std::cmp::{max, min};

use crossterm::{
    execute,
    terminal::{Clear, ClearType},
};

use crate::screen::viewnode::LayoutType;

use super::viewnode::{self, Border, BoxModelAttribute, ViewNode, ViewNodeType};

use std::io::stdout;

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
        print!("{}[2J", 27 as char);
        self.char_matrix.iter().for_each(|line| {
            line.iter().for_each(|c| {
                print!("{}", c);
            });
            println!();
        })
    }

    pub fn render_view_node_tree(&mut self, root: &RootViewNode) {
        Self::clear_screen();
        self.render_node_with_constraint(
            &root.0,
            &RenderConstraint(Coordinate2D(0, 0), Coordinate2D(self.height, self.width)),
        );
    }

    fn render_node_with_constraint(
        &mut self,
        view_node: &ViewNode,
        constraint: &RenderConstraint,
    ) -> RenderArea {
        let RenderConstraint(
            Coordinate2D(left_top_x, left_top_y),
            Coordinate2D(bottom_right_x, bottom_right_y),
        ) = constraint;

        let mut node_max_occupy_area = RenderArea(
            Coordinate2D(*left_top_x, *left_top_y),
            Coordinate2D(*left_top_x, *left_top_y),
        );

        // 渲染子节点
        match &view_node.node_type {
            ViewNodeType::LayoutType(layout_type) => {
                let mut inner_constraint =
                    Self::calculate_inner_constraint(&constraint, &view_node.box_model_attribute);

                // println!(
                //     "rendering layout node, type = {:?}, constraint = {:?}, inner constraint = {:?}",
                //     layout_type, constraint, inner_constraint
                // );

                view_node.child_nodes.iter().for_each(|childe_node| {
                    if inner_constraint.1 .0 > inner_constraint.0 .0
                        && inner_constraint.1 .1 > inner_constraint.0 .1
                    {
                        let current_occupy_area = match layout_type {
                            LayoutType::Box => {
                                self.render_node_with_constraint(childe_node, &inner_constraint)
                            }
                            LayoutType::Column => {
                                let current_occupy_area = self
                                    .render_node_with_constraint(childe_node, &inner_constraint);
                                inner_constraint.0 .0 = current_occupy_area.1 .0;
                                current_occupy_area
                            }
                            LayoutType::Row => {
                                let current_occupy_area = self
                                    .render_node_with_constraint(childe_node, &inner_constraint);
                                inner_constraint.0 .1 = current_occupy_area.1 .1;
                                current_occupy_area
                            }
                        };

                        node_max_occupy_area.1 .0 =
                            max(node_max_occupy_area.1 .0, current_occupy_area.1 .0);
                        node_max_occupy_area.1 .1 =
                            max(node_max_occupy_area.1 .1, current_occupy_area.1 .1);
                        // println!(
                        //     "rendering child node, inner_constraint = {:?}, occupy_area = {:?}",
                        //     inner_constraint, current_occupy_area
                        // );
                    }
                });
            }
            ViewNodeType::ContentType(content_type) => match content_type {
                viewnode::ContentType::Text(text) => {
                    // println!(
                    //     "redering text node, text = {}, constaint = {:?}, ",
                    //     text, constraint,
                    // );
                    for (index, c) in text.chars().enumerate() {
                        let x = *left_top_x;
                        let y = left_top_y + index;

                        if y < *bottom_right_y {
                            node_max_occupy_area.1 .0 = node_max_occupy_area.0 .0 + 1;
                            node_max_occupy_area.1 .1 = node_max_occupy_area.0 .1 + index + 1;
                            self.char_matrix[x][y] = c;
                        }
                    }
                }
            },
        };

        let mut node_width = match view_node.box_model_attribute.width {
            Some(width) => width,
            None => 0,
        };
        let mut node_height = match view_node.box_model_attribute.height {
            Some(height) => height,
            None => 0,
        };
        node_width = min(node_width, bottom_right_y - left_top_y);
        node_height = min(node_height, bottom_right_x - left_top_x);

        node_max_occupy_area.1 .0 = max(
            node_max_occupy_area.1 .0,
            node_max_occupy_area.0 .0 + node_height,
        );
        node_max_occupy_area.1 .1 = max(
            node_max_occupy_area.1 .1,
            node_max_occupy_area.0 .1 + node_width,
        );

        if view_node.box_model_attribute.border.is_none() {
            return node_max_occupy_area;
        }

        // 如果有边框，且空间足够就渲染边框

        let mut node_border = Border::new(0, 0, 0, 0);
        if let Some(border) = &view_node.box_model_attribute.border {
            node_border = Border::new(border.left, border.top, border.right, border.bottom)
        }

        for i in 0..node_width {
            if node_border.top > 0 {
                self.char_matrix[*left_top_x][left_top_y + i] = '-';
            }
            if node_border.bottom > 0 {
                self.char_matrix[left_top_x + node_height - 1][left_top_y + i] = '-';
            }
        }

        for i in 0..node_height {
            if node_border.left > 0 {
                self.char_matrix[left_top_x + i][*left_top_y] = '|';
            }
            if node_border.bottom > 0 {
                self.char_matrix[left_top_x + i][left_top_y + node_width - 1] = '|';
            }
        }

        node_max_occupy_area
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

        let outer_width = match box_model_attribute.width {
            Some(width) => width as u32,
            _ => u32::MAX,
        };
        let outer_height = match box_model_attribute.height {
            Some(height) => height as u32,
            _ => u32::MAX,
        };

        let inner_left_top_x = outer_constraint.0 .0 as u32 + border_top + padding_top;
        let inner_left_top_y = outer_constraint.0 .1 as u32 + border_left + padding_left;
        let inner_bottom_right_x = min(
            outer_constraint.0 .0 as u32 + outer_height,
            outer_constraint.1 .0 as u32,
        ) - border_bottom
            - padding_bottom;
        let inner_bottom_right_y = min(
            outer_constraint.0 .1 as u32 + outer_width,
            outer_constraint.1 .1 as u32,
        ) - border_right
            - padding_right;

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

    fn clear_screen() {
        let mut stdout = stdout();
        execute!(stdout, Clear(ClearType::All)).unwrap();
    }
}

pub struct RootViewNode(pub ViewNode);

#[derive(Debug)]
pub struct Coordinate2D(pub usize, pub usize);
#[derive(Debug)]
struct RenderArea(Coordinate2D, Coordinate2D);

#[derive(Debug)]
struct RenderConstraint(Coordinate2D, Coordinate2D);
