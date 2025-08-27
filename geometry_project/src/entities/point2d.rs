use crate::{
    display::{scene::Scene, scene_proxy::ISceneProxy},
    entities::affine_matrix2d::Column,
    numerics::floating_comparisons::approx_equal,
};
use core::f32;
use serde::{Deserialize, Serialize};
use std::{
    fmt,
    ops::{Mul, Sub},
};

use super::{affine_matrix2d::Matrix2D, vect2d::Vector2D};

#[derive(Copy, Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Point2d {
    pub x: f32,
    pub y: f32,
}

impl Into<(f32, f32)> for Point2d
{
    fn into(self) -> (f32, f32) {
        (self.x, self.y)
    }
}

impl fmt::Display for Point2d {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

impl ISceneProxy for Point2d {
    fn get_scene(&self) -> crate::display::scene::Scene {
        let mut scene = Scene::new();
        scene.push_point(self.clone());

        scene
    }
}

impl Into<Column> for Point2d {
    fn into(self) -> Column {
        Column {
            r1: self.x,
            r2: self.y,
            r3: 1f32,
        }
    }
}

impl From<Column> for Point2d {
    fn from(value: Column) -> Self {
        Point2d {
            x: value.r1,
            y: value.r2,
        }
    }
}

impl Sub<Point2d> for Point2d {
    type Output = Vector2D;
    fn sub(self, rhs: Point2d) -> Self::Output {
        Vector2D {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

impl Mul<Matrix2D> for Point2d {
    type Output = Point2d;

    fn mul(self, rhs: Matrix2D) -> Self::Output {
        let col: Column = self.into();
        (col * rhs).into()
    }
}

impl Point2d {
    pub fn origin() -> Self {
        Point2d { x: 0f32, y: 0f32 }
    }

    pub fn new(x: f32, y: f32) -> Self {
        Point2d { x, y }
    }

    pub fn nan() -> Self {
        Point2d {
            x: f32::NAN,
            y: f32::NAN,
        }
    }

    pub fn approx_equals(&self, other: &Point2d, epsilon: f32) -> bool {
        approx_equal(self.x, other.x, epsilon) && approx_equal(self.y, other.y, epsilon)
    }

    pub fn is_nan(&self) -> bool {
        self.x.is_nan() || self.y.is_nan()
    }
}
