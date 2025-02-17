use std::ops::{Add, Mul};

use super::point2d::Point2d;

#[derive(Debug, Clone, Copy)]
pub struct Vector2D {
    pub x: f32,
    pub y: f32,
}

impl From<Point2d> for Vector2D {
    fn from(value: Point2d) -> Self {
        Vector2D {
            x: value.x,
            y: value.y,
        }
    }
}

impl Add<Vector2D> for Vector2D {
    type Output = Vector2D;

    fn add(self, rhs: Vector2D) -> Self::Output {
        Vector2D {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl Into<Point2d> for Vector2D {
    fn into(self) -> Point2d {
        Point2d {
            x: self.x,
            y: self.y,
        }
    }
}

impl Mul<Vector2D> for Vector2D {
    type Output = Vector2D;

    fn mul(self, rhs: Vector2D) -> Self::Output {
        Vector2D {
            x: self.x * rhs.x,
            y: self.y * rhs.y,
        }
    }
}

impl Vector2D {
    pub fn translate_point(&self, point: Point2d) -> Point2d {
        Point2d {
            x: point.x + self.x,
            y: point.y + self.y,
        }
    }

    pub fn len(&self) -> f32 {
        f32::sqrt(f32::powi(self.x, 2) + f32::powi(self.y, 2))
    }
}
