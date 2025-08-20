use serde::{Deserialize, Serialize};

use crate::display::rgb::RGB;
use crate::entities::{line2d::Line2D, point2d::Point2d};

#[derive(Serialize, Deserialize, Debug)]
pub struct Scene {
    pub points: Vec<Point2d>,
    pub lines: Vec<Line2D>,
    pub point_colors: Vec<RGB>,
    pub line_colors: Vec<RGB>,
}

impl Scene {
    pub fn new() -> Self {
        Scene {
            points: Vec::new(),
            lines: Vec::new(),
            point_colors: Vec::new(),
            line_colors: Vec::new(),
        }
    }

    pub fn from_lines(lines: Vec<Line2D>) -> Self {
        Scene {
            points: Vec::new(),
            lines,
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

    pub fn push_line(&mut self, line: Line2D) {
        self.lines.push(line);
    }

    pub fn push_point(&mut self, point: Point2d) {
        self.points.push(point);
    }

    pub fn merge(&mut self, mut other: Scene) {
        self.lines.append(&mut other.lines);
        self.points.append(&mut other.points);
        self.point_colors.append(&mut other.point_colors);
        self.line_colors.append(&mut other.line_colors);
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

    pub fn push_line_color(&mut self, line_color: RGB) {
        self.line_colors.push(line_color);
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
