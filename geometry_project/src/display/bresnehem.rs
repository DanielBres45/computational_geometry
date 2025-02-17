use memory_math::memory_index2d::MemIndex2D;

use crate::entities::{line2d::Line2D, point2d::Point2d};

pub struct BresnehemIter {
    start: Point2d,
    end: Point2d,
    dx: f32,
    dy: f32,
    sx: f32,
    sy: f32,
    error: f32,
}

impl From<Line2D> for BresnehemIter {
    fn from(value: Line2D) -> Self {
        let dx: f32 = f32::abs(value.end.x - value.start.x);
        let sx: f32 = match value.start.x < value.end.x {
            true => 1f32,
            false => -1f32,
        };

        let dy: f32 = -f32::abs(value.end.y - value.start.y);
        let sy: f32 = match value.start.y < value.end.y {
            true => 1f32,
            false => -1f32,
        };

        let error: f32 = dx + dy;

        BresnehemIter {
            start: value.start,
            end: value.end,
            dx,
            dy,
            sx,
            sy,
            error,
        }
    }
}

impl Iterator for BresnehemIter {
    type Item = MemIndex2D;

    fn next(&mut self) -> Option<Self::Item> {
        if 2f32 * self.error >= self.dy {
            if self.start.x == self.end.x {
                return None;
            }

            self.error += self.dy;
            self.start = Point2d {
                x: self.start.x + self.sx,
                y: self.start.y,
            };
        }

        if 2f32 * self.error <= self.dx {
            if self.end.y == self.start.y {
                return None;
            }

            self.error += self.dx;
            self.start = Point2d {
                x: self.start.x,
                y: self.start.y + self.sy,
            };
        }

        Some(MemIndex2D::new(
            self.start.y as usize,
            self.start.x as usize,
        ))
    }
}
