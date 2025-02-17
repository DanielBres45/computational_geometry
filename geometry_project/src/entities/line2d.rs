use core::fmt;
use std::ops::Mul;

use crate::entities::point2d::Point2d;

use super::affine_matrix2d::{Column, Matrix2D};

#[derive(Debug, Clone, Copy)]
pub struct Line2D {
    pub start: Point2d,
    pub end: Point2d,
}

impl fmt::Display for Line2D {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[{}, {}]", self.start, self.end)
    }
}

impl Mul<Matrix2D> for Line2D {
    type Output = Line2D;

    fn mul(self, rhs: Matrix2D) -> Self::Output {
        let col1: Column = self.start.into();
        let col2: Column = self.end.into();

        let new_start: Point2d = (col1 * rhs).into();
        let new_end: Point2d = (col2 * rhs).into();

        Line2D::new(new_start, new_end)
    }
}

impl Line2D {
    pub fn new(start: Point2d, end: Point2d) -> Self {
        Line2D { start, end }
    }

    pub fn new_flat(x1: f32, y1: f32, x2: f32, y2: f32) -> Self {
        Line2D {
            start: Point2d { x: x1, y: y1 },
            end: Point2d { x: x2, y: y2 },
        }
    }

    pub fn len(&self) -> f32 {
        (self.start - self.end).len()
    }

    pub fn approx_equals(&self, other: &Line2D, epsilon: f32) -> bool {
        (self.start.approx_equals(&other.start, epsilon)
            && self.end.approx_equals(&other.end, epsilon))
            || (self.start.approx_equals(&other.end, epsilon)
                && self.end.approx_equals(&other.start, epsilon))
    }
}
