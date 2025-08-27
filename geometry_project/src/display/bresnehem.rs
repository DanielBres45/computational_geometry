use std::fmt::Debug;

use memory_math::memory_index2d::MemIndex2D;

use crate::{
    entities::{line2d::Line2D, point2d::Point2d},
    numerics::floating_comparisons::approx_equal,
};

#[derive(Clone, Copy)]
pub struct BresnehemIter {
    pub start: Point2d,
    pub end: Point2d,
    dx: f32,
    dy: f32,
    sx: f32,
    sy: f32,
    error: f32,
}

impl Debug for BresnehemIter
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("BresnehemIter").field("start", &self.start).field("end", &self.end).field("dx", &self.dx).field("dy", &self.dy).field("sx", &self.sx).field("sy", &self.sy).field("error", &self.error).finish()
    }
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

        //println!("{:?}", &self);
        //println!("point: {}", self.start);

        let value: Option<MemIndex2D> = Some(MemIndex2D::new(
            self.start.y as usize,
            self.start.x as usize,
        ));

        if self.start.approx_equals(&self.end, 0.1) {
            self.start = Point2d::nan();
            return value;
        }

        if 2f32 * self.error >= self.dy {
            if approx_equal(self.start.x, self.end.x, 0.1) {
                self.start = Point2d::nan();
                return value;
            }

            self.error += self.dy;
            self.start = Point2d {
                x: self.start.x + self.sx,
                y: self.start.y,
            };
        }

        if 2f32 * self.error <= self.dx {
            if approx_equal(self.end.y, self.start.y, 0.1) {
                self.start = Point2d::nan();
                return value;
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

    #[test]
    fn test_no_zero() {
        let line: Line2D = Line2D::new_flat(383.45f32, 260.17f32, 378.096f32, 180.92f32);

        let iter: BresnehemIter = line.into();

        for mem_index in iter {
            println!("index: {}", mem_index);
            assert_ne!(MemIndex2D { row: 0, col: 0 }, mem_index);
        }
    }

    #[test]
    fn test_specific() {
        let line: Line2D = Line2D { start: Point2d { x: 41.734604, y: 30.496841 }, end: Point2d { x: 52.04078, y: 27.98661 } }; 
        let iter: BresnehemIter = line.into();

        let mut idx: usize = 0;
        for mem_index in iter
        {
            println!("Index: {}", mem_index);
            assert!(
                idx < 30 as usize
            );
            idx += 1;
        }
    }
}
