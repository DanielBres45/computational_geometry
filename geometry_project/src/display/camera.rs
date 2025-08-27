use core::{f32, panic};

use crate::display::rgb::RGB;
use crate::entities::affine_matrix2d::Matrix2D;
use crate::entities::algorithms;
use crate::entities::polygon2d::Polygon2D;
use crate::entities::rectangle2d::Rectangle2D;
use crate::entities::vect2d::Vector2D;
use crate::entities::{line2d::Line2D, point2d::Point2d};
use crate::numerics::floating_comparisons::approx_equal;
use data_structures::vec2d::Vec2D;
use log_statement::def_log;
use memory_math::memory_index2d::MemIndex2D;
use memory_math::memory_line::MemLine2D;

use super::bresnehem::BresnehemIter;

//TODO: Implement point size = points into squares
pub struct Camera {
    pub view_port: Rectangle2D,
    points: Vec<Point2d>,
    lines: Vec<Line2D>,
    point_colors: Vec<RGB>,
    line_colors: Vec<RGB>,
    point_size: u8
}

const LOGGING_ENABLED: bool = false;
def_log!(Camera, LOGGING_ENABLED);

impl Camera {
    pub fn new(width: f32, height: f32) -> Self {
        Camera {
            view_port: Rectangle2D::new_width_height(width, height),
            points: Vec::new(),
            lines: Vec::new(),
            point_colors: Vec::new(),
            line_colors: Vec::new(),
            point_size: 1
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

    fn draw_line(&self, line: Line2D, color: RGB, canvas: &mut Vec2D<RGB>) {

        camera_log!("Plotting Line {:?}", line);
        let min: MemIndex2D;
        if let Ok(m) = MemIndex2D::try_from(Into::<(f32, f32)>::into(line.start))
        {
            min = m;
        }
        else {
            camera_log!("Could not plot line {:?}", line);
            return;
        }

        let max: MemIndex2D;
        if let Ok(m) = MemIndex2D::try_from(Into::<(f32, f32)>::into(line.end))
        {
            max = m;
        }
        else {
            camera_log!("Could not plot line {:?}", line);
            return;
        }

        let indexes: Vec<MemIndex2D> = MemLine2D::new(min, max).line_indexes();

        camera_log!("Plotting line indexes: {:?}", indexes);
        for point in indexes {
            if canvas.index2d_in_bounds(point) {
                canvas[point] = color;
            }
        }
    }

    fn draw_lines(&self, canvas: &mut Vec2D<RGB>, skew: Matrix2D) {
        camera_log!("Drawing lines: {:?}", || &self.lines);

        let mut idx: usize = 0;
        for line in &self.lines {
            //println!("Draw line at {}", line);
            let clipped: Line2D = match self.clip_line(line.to_owned()) {
                Some(l) => l,
                None => {
                    continue;
                }
            };

            if approx_equal(clipped.len(), 0f32, f32::EPSILON) {
                continue;
            }

            let scaled_line: Line2D = clipped * skew;

            let color: RGB = match self.line_colors.len() > idx {
                true => self.line_colors[idx],
                false => RGB::red(),
            };
            idx += 1;

            self.draw_line(scaled_line, color, canvas);
        }
    }

    fn draw_points(&self, canvas: &mut Vec2D<RGB>, skew: Matrix2D)
    {
        camera_log!("Drawing points: {:?}", || &self.points);
        for i in 0..self.points.len(){

            if !self.view_port.contains_closed(&self.points[i], f32::EPSILON)
            {
                continue;
            }

            let color: RGB = match i < self.point_colors.len()
            {
                true => self.point_colors[i].clone(),
                false => RGB::white()
            };

            let normalized: Point2d = self.points[i].clone() * skew;

            if self.point_size == 1
            {
                canvas[Self::point_into_index(normalized)] = color;
                continue;
            }

            let coord: MemIndex2D = Self::point_into_index(normalized);
            let row_min: usize = coord.row.checked_sub((self.point_size / 2) as usize).unwrap_or(0);
            let col_min: usize = coord.col.checked_sub((self.point_size / 2) as usize).unwrap_or(0);

            let row_max: usize = usize::min(canvas.height(), coord.row + (self.point_size - self.point_size / 2) as usize);
            let col_max: usize = usize::min(canvas.width(), coord.col + (self.point_size - self.point_size / 2) as usize);
        
            for row in row_min..row_max
            {
                for col in col_min..col_max
                {
                    canvas[MemIndex2D::new(row, col)] = color;
                }
            }
        }
    }

    pub fn draw(&self, canvas: &mut Vec2D<RGB>) {
        camera_log!("Starting draw");
        
        let scale: Vector2D = Vector2D {
            x: canvas.width() as f32 / self.view_port.width(),
            y: canvas.height() as f32 / self.view_port.height(),
        };

        let translate: Vector2D = Vector2D::from(self.view_port.min);

        let skew: Matrix2D = Matrix2D::skew(translate, scale);

        camera_log!("Skew, {:?}", || skew);

        self.draw_lines(canvas, skew);
        self.draw_points(canvas, skew);
    }

    pub fn set_point_size(&mut self, point_size: u8)
    {
        self.point_size = point_size;
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

    pub fn set_point_color(&mut self, color: RGB) {
        for _ in 0..self.points.len() {
            self.point_colors.push(color.clone());
        }
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
