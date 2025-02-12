use core::panic;

use crate::display::rgb::RGB;
use crate::entities::affine_matrix2d::Matrix2D;
use crate::entities::rectangle2d::Rectangle2D;
use crate::entities::vect2d::Vector2D;
use crate::entities::{line2d::Line2D, point2d::Point2d};

use data_structures::vec2d::Vec2D;
use memory_math::memory_index2d::MemIndex2D;

pub struct Camera {
    pub view_port: Rectangle2D,
    points: Vec<Point2d>,
    lines: Vec<Line2D>,
    point_colors: Vec<RGB>,
    line_colors: Vec<RGB>,
}

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
