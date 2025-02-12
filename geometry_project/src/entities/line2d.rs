use crate::entities::point2d::Point2d;

pub struct Line2D {
    pub start: Point2d,
    pub end: Point2d,
}

impl Line2D {
    pub fn new(start: Point2d, end: Point2d) -> Self {
        Line2D { start, end }
    }
}
