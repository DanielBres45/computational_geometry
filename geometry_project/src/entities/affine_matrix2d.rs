use crate::entities::vect2d::Vector2D;

use std::ops::Mul;

#[derive(Debug, Clone, Copy)]
pub struct Matrix2D {
    r1: Row,
    r2: Row,
    r3: Row,
}

#[derive(Debug, Clone, Copy)]
pub struct Row {
    pub c1: f32,
    pub c2: f32,
    pub c3: f32,
}

#[derive(Debug, Clone, Copy)]
pub struct Column {
    pub r1: f32,
    pub r2: f32,
    pub r3: f32,
}

impl Mul<Matrix2D> for Column {
    type Output = Self;

    fn mul(self, rhs: Matrix2D) -> Self::Output {
        Column {
            r1: self * rhs.r1,
            r2: self * rhs.r2,
            r3: self * rhs.r3,
        }
    }
}

impl Mul<Row> for Column {
    type Output = f32;

    fn mul(self, rhs: Row) -> Self::Output {
        self.r1 * rhs.c1 + self.r2 * rhs.c2 + self.r3 * rhs.c3
    }
}

impl Matrix2D {
    pub fn skew(translate: Vector2D, scale: Vector2D) -> Self {
        Matrix2D {
            r1: Row {
                c1: scale.x,
                c2: 0f32,
                c3: translate.x,
            },
            r2: Row {
                c1: 0f32,
                c2: scale.y,
                c3: translate.y,
            },
            r3: Row {
                c1: 0f32,
                c2: 0f32,
                c3: 1f32,
            },
        }
    }

    pub fn translation(vec: Vector2D) -> Self {
        Matrix2D {
            r1: Row {
                c1: 1f32,
                c2: 0f32,
                c3: vec.x,
            },
            r2: Row {
                c1: 0f32,
                c2: 1f32,
                c3: vec.y,
            },
            r3: Row {
                c1: 0f32,
                c2: 0f32,
                c3: 0f32,
            },
        }
    }
}
