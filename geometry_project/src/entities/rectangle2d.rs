use std::fmt;

use crate::{
    entities::point2d::Point2d,
    numerics::floating_comparisons::{
        self, approx_equal, approx_equal_greater, approx_equal_less, approx_less,
    },
};

#[derive(Copy, Clone, Debug)]
pub struct Rectangle2D {
    pub min: Point2d,
    pub max: Point2d,
}

impl fmt::Display for Rectangle2D {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "(Min: {}, Max: {})", self.min, self.max)
    }
}

impl Rectangle2D {
    pub fn new_width_height(width: f32, height: f32) -> Self {
        Rectangle2D {
            min: Point2d::origin(),
            max: Point2d {
                x: width,
                y: height,
            },
        }
    }

    pub fn is_nan(&self) -> bool {
        self.min.is_nan() || self.max.is_nan()
    }

    pub fn width(&self) -> f32 {
        f32::abs(self.max.x - self.min.x)
    }

    pub fn height(&self) -> f32 {
        f32::abs(self.max.y - self.min.y)
    }

    pub fn contains_open(&self, point: Point2d, epsilon: f32) -> bool {
        if approx_equal_less(point.x, self.min.x, epsilon)
            || approx_equal_less(point.y, self.min.y, epsilon)
        {
            return false;
        }

        return approx_less(point.x, self.max.x, epsilon)
            && approx_less(point.y, self.max.y, epsilon);
    }

    pub fn contains_closed(&self, point: &Point2d, epsilon: f32) -> bool {
        approx_equal_greater(point.x, self.min.x, epsilon)
            && approx_equal_greater(point.y, self.min.y, epsilon)
            && approx_equal_less(point.y, self.max.y, epsilon)
            && approx_equal_less(point.x, self.max.x, epsilon)
    }
}
