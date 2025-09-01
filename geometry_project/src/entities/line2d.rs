use core::fmt;
use std::{f32, ops::Mul};

use serde::{Deserialize, Serialize};

use crate::{
    entities::{point2d::Point2d, vect2d::Vector2D},
    numerics::{approx_equatable::ApproxEquals, approx_partial_order::ApproxPartialOrder},
};

use super::affine_matrix2d::{Column, Matrix2D};

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct Line2D {
    pub start: Point2d,
    pub end: Point2d,
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy)]
pub struct ParametricLine2D(pub Line2D);

impl From<Line2D> for ParametricLine2D {
    fn from(value: Line2D) -> Self {
        Self(value)
    }
}

impl fmt::Display for ParametricLine2D {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({} + t*{}, {} + t*{})", 
        self.0.start.x, self.0.direction().x,
        self.0.start.y, self.0.direction().y)
    }
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

    pub fn nan() -> Line2D {
        Line2D {
            start: Point2d::nan(),
            end: Point2d::nan(),
        }
    }

    pub fn direction(&self) -> Vector2D
    {
        &self.end - &self.start
    }

    pub fn reverse(&self) -> Self
    {
        Line2D { start: self.end, end: self.start }
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

    pub fn is_finite(&self) -> bool {
        !self.is_nan()
        && self.start.is_finite()
        && self.end.is_finite()
    }

    pub fn intersects_point(&self, point: Point2d, epsilon: f32) -> bool {
        let dp = point - self.start;
        let lp = self.end - self.start;

        dp.cross(&lp).approx_equals(&0f32, epsilon)
    }

    ///See stack overflow post https://stackoverflow.com/questions/563198/how-do-you-detect-where-two-line-segments-intersect
    pub fn intersect(&self, other: &Line2D, epsilon: f32) -> Option<Point2d> {
        if self.is_nan() || other.is_nan() {
            return None;
        }

        if self.len().approx_equals(&0f32, epsilon) || other.len().approx_equals(&0f32, epsilon) {
            return None;
        }

        let s1_x: f32 = self.end.x - self.start.x;
        let s1_y: f32 = self.end.y - self.start.y;

        let s2_x: f32 = other.end.x - other.start.x;
        let s2_y: f32 = other.end.y - other.start.y;

        let s: f32 = (-s1_y * (self.start.x - other.start.x)
            + s1_x * (self.start.y - other.start.y))
            / (-s2_x * s1_y + s1_x * s2_y);

        let t: f32 = (s2_x * (self.start.y - other.start.y)
            - s2_y * (self.start.x - other.start.x))
            / (-s2_x * s1_y + s1_x * s2_y);

        if s.approx_equals_or_greater_than(&0f32, epsilon)
            && s.approx_equals_or_less_than(&1f32, epsilon)
            && t.approx_equals_or_greater_than(&0f32, epsilon)
            && t.approx_equals_or_less_than(&1f32, epsilon)
        {
            return Some(Point2d::new(
                self.start.x + (t * s1_x),
                self.start.y + (t * s1_y),
            ));
        }

        return None;
    }
}

#[cfg(test)]
mod tests {
    use std::f32;

    use super::*;

    const EPSILON: f32 = f32::EPSILON;

    #[test]
    fn test_approx_equal_1() {
        let a: Line2D = Line2D::new_flat(0f32, 0f32, 10f32, 10f32);
        let b: Line2D = Line2D::new_flat(0f32, 0f32, 10f32, 10f32);

        assert!(a.approx_equals(&b, EPSILON));
    }

    #[test]
    fn test_approx_equal_2() {
        let a: Line2D = Line2D::new_flat(0f32, 0f32, 10f32, 10f32);
        let b: Line2D = Line2D::new_flat(10f32, 10f32, 0f32, 0f32);

        assert!(a.approx_equals(&b, EPSILON));
    }

    fn intersection_string(maybe_point: Option<Point2d>) -> String {
        maybe_point.map_or_else(|| "None".to_string(), |p| format!("{}", p))
    }

    #[test]
    fn test_intersect_1() {
        let a: Line2D = Line2D::new_flat(0f32, 0f32, 10f32, 10f32);
        let b: Line2D = Line2D::new_flat(5f32, 5f32, 5f32, -5f32);

        let expected: Point2d = Point2d::new(5f32, 5f32);

        let intersect: Option<Point2d> = a.intersect(&b, EPSILON);

        assert!(
            intersect.is_some_and(|p| p.approx_equals(&expected, EPSILON)),
            "Expected intersection was (5,0) but actual was {}",
            intersection_string(intersect)
        );
    }

    #[test]
    fn test_intersect_2() {
        let a: Line2D = Line2D::new_flat(0f32, 0f32, 0f32, 10f32);
        let b: Line2D = Line2D::new_flat(0f32, 10f32, 10f32, 10f32);

        let expected: Point2d = Point2d::new(0f32, 10f32);
        let intersection: Option<Point2d> = a.intersect(&b, EPSILON);

        assert!(
            intersection.is_some_and(|p| p.approx_equals(&expected, EPSILON)),
            "Expected intersection was (0,0) but actual was {}",
            intersection_string(intersection)
        );
    }
}
