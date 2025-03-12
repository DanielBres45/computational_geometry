use crate::entities::{point2d::Point2d, rectangle2d::Rectangle2D};
use rand::random_range;

pub struct Random2D {}

impl Random2D {
    pub fn random_point(range: Rectangle2D) -> Point2d {
        if range.is_nan() {
            return Point2d::nan();
        }

        let x = random_range(range.min.x..range.max.x);
        let y = random_range(range.min.y..range.max.y);

        Point2d { x, y }
    }

    pub fn random_points(range: Rectangle2D, count: i32) -> RandomPoints {
        RandomPoints {
            max: count,
            bounds: range,
        }
    }
}

pub struct RandomPoints {
    max: i32,
    bounds: Rectangle2D,
}

impl IntoIterator for RandomPoints {
    type Item = Point2d;
    type IntoIter = RandomPointsIter;

    fn into_iter(self) -> Self::IntoIter {
        RandomPointsIter {
            idx: self.max,
            bounds: self.bounds,
        }
    }
}

pub struct RandomPointsIter {
    idx: i32,
    bounds: Rectangle2D,
}

impl Iterator for RandomPointsIter {
    type Item = Point2d;

    fn next(&mut self) -> Option<Self::Item> {
        if self.idx <= 0 {
            return None;
        }

        self.idx -= 1;
        Some(Random2D::random_point(self.bounds))
    }
}
