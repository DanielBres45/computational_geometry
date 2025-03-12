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
        if self.start.is_nan() {
            return None;
        }

        let value: Option<MemIndex2D> = Some(MemIndex2D::new(
            self.start.y as usize,
            self.start.x as usize,
        ));

        if 2f32 * self.error >= self.dy {
            if self.start.x == self.end.x {
                self.start = Point2d::nan();
            }

            self.error += self.dy;
            self.start = Point2d {
                x: self.start.x + self.sx,
                y: self.start.y,
            };
        }

        if 2f32 * self.error <= self.dx {
            if self.end.y == self.start.y {
                self.start = Point2d::nan();
            }

            self.error += self.dx;
            self.start = Point2d {
                x: self.start.x,
                y: self.start.y + self.sy,
            };
        }

        value
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_line_1() {
        let line: Line2D = Line2D::new_flat(0f32, 0f32, 10f32, 0f32);
        let iter: BresnehemIter = line.into();
        let expected: [MemIndex2D; 10] = [
            MemIndex2D::new(0, 0),
            MemIndex2D::new(0, 1),
            MemIndex2D::new(0, 2),
            MemIndex2D::new(0, 3),
            MemIndex2D::new(0, 4),
            MemIndex2D::new(0, 5),
            MemIndex2D::new(0, 6),
            MemIndex2D::new(0, 7),
            MemIndex2D::new(0, 8),
            MemIndex2D::new(0, 9),
        ];

        let list: Vec<MemIndex2D> = iter.collect();

        for i in 0..10 {
            println!("{},expected: {}, list: {}", i, expected[i], list[i]);
            assert_eq!(expected[i], list[i])
        }
    }

    #[test]
    fn test_line_2() {
        let line: Line2D = Line2D::new_flat(0f32, 0f32, 0f32, 10f32);
        let iter: BresnehemIter = line.into();
        let expected: [MemIndex2D; 10] = [
            MemIndex2D::new(0, 0),
            MemIndex2D::new(1, 0),
            MemIndex2D::new(2, 0),
            MemIndex2D::new(3, 0),
            MemIndex2D::new(4, 0),
            MemIndex2D::new(5, 0),
            MemIndex2D::new(6, 0),
            MemIndex2D::new(7, 0),
            MemIndex2D::new(8, 0),
            MemIndex2D::new(9, 0),
        ];

        let list: Vec<MemIndex2D> = iter.collect();

        for i in 0..10 {
            println!("{}, expected: {}, list: {}", i, expected[i], list[i]);
            assert_eq!(expected[i], list[i])
        }
    }
}
