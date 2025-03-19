use core::fmt;
use std::ops::Mul;

use crate::entities::point2d::Point2d;

use super::affine_matrix2d::{Column, Matrix2D};

#[derive(Debug, Clone, Copy)]
pub struct Line2D {
    pub start: Point2d,
    pub end: Point2d,
}

impl PartialEq for Line2D {
    fn eq(&self, other: &Self) -> bool {
        (self.start.eq(&other.start) && self.end.eq(&other.end))
            || (self.end.eq(&other.start) && self.start.eq(&other.end))
    }
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

    pub fn new_flat_int(x1: i32, y1: i32, x2: i32, y2: i32) -> Self {
        Self::new_flat(x1 as f32, y1 as f32, x2 as f32, y2 as f32)
    }

    pub fn new_flat_into<T: Into<f32>>(x1: T, y1: T, x2: T, y2: T) -> Self {
        Self::new_flat(x1.into(), y1.into(), x2.into(), y2.into())
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

    pub fn is_nan(&self) -> bool {
        self.start.is_nan() || self.end.is_nan()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_approx_equal_1() {
        let a: Line2D = Line2D::new_flat(0f32, 0f32, 10f32, 10f32);
        let b: Line2D = Line2D::new_flat(0f32, 0f32, 10f32, 10f32);

        assert!(a.approx_equals(&b, f32::EPSILON));
    }

    #[test]
    fn test_approx_equal_2() {
        let a: Line2D = Line2D::new_flat(0f32, 0f32, 10f32, 10f32);
        let b: Line2D = Line2D::new_flat(10f32, 10f32, 0f32, 0f32);

        assert!(a.approx_equals(&b, f32::EPSILON));
    }
}
