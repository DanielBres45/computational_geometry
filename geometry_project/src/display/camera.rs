use core::panic;

use crate::display::rgb::RGB;
use crate::entities::affine_matrix2d::Matrix2D;
use crate::entities::algorithms;
use crate::entities::polygon2d::Polygon2D;
use crate::entities::rectangle2d::Rectangle2D;
use crate::entities::vect2d::Vector2D;
use crate::entities::{line2d::Line2D, point2d::Point2d};
use crate::numerics::floating_comparisons::approx_equal;

use log_statement::def_log;

use data_structures::vec2d::Vec2D;
use memory_math::memory_index2d::MemIndex2D;

use super::bresnehem::BresnehemIter;

pub struct Camera {
    pub view_port: Rectangle2D,
    points: Vec<Point2d>,
    lines: Vec<Line2D>,
    point_colors: Vec<RGB>,
    line_colors: Vec<RGB>,
}

def_log!(Camera);

impl Camera {
    pub fn new(width: f32, height: f32) -> Self {
        Camera {
            view_port: Rectangle2D::new_width_height(width, height),
            points: Vec::new(),
            lines: Vec::new(),
            point_colors: Vec::new(),
            line_colors: Vec::new(),
        }
    }

    pub fn clear(&mut self) {
        self.points.clear();
        self.lines.clear();
        self.point_colors.clear();
        self.line_colors.clear();
    }

    fn point_into_index(value: Point2d) -> MemIndex2D {
        if !value.y.is_finite() || !value.x.is_finite() {
            panic!("point not finite: {}", value);
        }

        if value.y < 0f32 || value.x < 0f32 {
            panic!("Point is negative");
        }

        let row: usize = value.y as usize;

        let col: usize = value.x as usize;

        MemIndex2D::new(row, col)
    }

    fn clip_line(&self, line: Line2D) -> Option<Line2D> {
        algorithms::liang_barsky_clip(self.view_port, line)
    }

    fn draw_line(&self, line: Line2D, canvas: &mut Vec2D<RGB>) {
        let iter: BresnehemIter = line.into();

        for point in iter {
            if canvas.index2d_in_bounds(point) {
                canvas[point] = RGB::red();
            }
        }
    }

    fn draw_lines(&self, canvas: &mut Vec2D<RGB>, skew: Matrix2D) {
        //println!("draw lines");
        for line in &self.lines {
            //println!("Draw line at {}", line);
            let clipped: Line2D = match self.clip_line(line.to_owned()) {
                Some(l) => l,
                None => {
                    //camera_log!("not in view!");
                    continue;
                }
            };

            //println!("Line after clipping: {}", clipped);

            if approx_equal(clipped.len(), 0f32, f32::EPSILON) {
                //camera_log!("0 length line. skipping");
                continue;
            }

            let scaled_line: Line2D = clipped * skew;

            //println!("Line after skew: {}", scaled_line);

            self.draw_line(scaled_line, canvas);
        }
    }

    pub fn draw(&self, canvas: &mut Vec2D<RGB>) {
        let scale: Vector2D = Vector2D {
            x: canvas.width() as f32 / self.view_port.width(),
            y: canvas.height() as f32 / self.view_port.height(),
        };

        let translate: Vector2D = Vector2D::from(self.view_port.min);

        let skew: Matrix2D = Matrix2D::skew(translate, scale);

        for point in &self.points {
            if self.view_port.contains_closed(point.clone(), f32::EPSILON) {
                let normalized: Point2d = point.clone() * skew;

                canvas[Self::point_into_index(normalized)] = RGB::white();
            }
        }

        self.draw_lines(canvas, skew);
    }

    pub fn push_polygon(&mut self, polygon: Polygon2D) {
        for line in polygon.lines() {
            self.push_line(line);
        }
    }

    pub fn push_line(&mut self, line: Line2D) {
        self.lines.push(line);
    }

    pub fn push_point(&mut self, point: Point2d) {
        self.points.push(point);
    }

    pub fn push_points<T>(&mut self, points: T)
    where
        T: IntoIterator<Item = Point2d>,
    {
        for point in points {
            self.points.push(point);
        }
    }

    pub fn push_point_colors<T>(&mut self, points_colors: T)
    where
        T: IntoIterator<Item = (Point2d, RGB)>,
    {
        for (point, color) in points_colors {
            self.points.push(point);
            self.point_colors.push(color);
        }
    }

    pub fn push_lines<T>(&mut self, lines: T)
    where
        T: IntoIterator<Item = Line2D>,
    {
        for line in lines {
            self.lines.push(line);
        }
    }

    pub fn push_lines_color<T>(&mut self, lines_colors: T)
    where
        T: IntoIterator<Item = (Line2D, RGB)>,
    {
        for (line, color) in lines_colors {
            self.lines.push(line);
            self.line_colors.push(color);
        }
    }
}
