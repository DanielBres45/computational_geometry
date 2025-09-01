use crate::entities::{line2d::Line2D, point2d::Point2d, rectangle2d::Rectangle2D};
use rand::random_range;

pub struct Random2D {}

impl Random2D {
    
    //Get random point within rectangle
    pub fn random_point(range: &Rectangle2D) -> Point2d {
        if range.is_nan() {
            return Point2d::nan();
        }

        let x = random_range(range.min.x..range.max.x);
        let y = random_range(range.min.y..range.max.y);
        Point2d { x, y }
    }

    pub fn random_point_int(range: &Rectangle2D) -> Point2d {
        if range.is_nan() {
            return Point2d::nan();
        }

        let x = random_range(range.min.x..range.max.x).round();
        let y = random_range(range.min.y..range.max.y).round();

        Point2d { x, y }
    }

    pub fn random_line_int(range: &Rectangle2D) -> Line2D {
        if range.is_nan() {
            return Line2D::nan();
        }

        let start: Point2d = Random2D::random_point_int(range);
        let end: Point2d = Random2D::random_point_int(range);

        Line2D::new(start, end)
    }

    pub fn random_line(range: &Rectangle2D) -> Line2D {
        if range.is_nan() {
            return Line2D::nan();
        }

        let start: Point2d = Random2D::random_point(range);
        let end: Point2d = Random2D::random_point(range);

        Line2D::new(start, end)
    }

    pub fn random_lines(range: Rectangle2D, count: i32) -> RandomEntities<Line2D> {
        RandomEntities {
            max: count,
            bounds: range,
            generator: Random2D::random_line as fn(&Rectangle2D) -> Line2D,
        }
    }

    pub fn random_lines_int(range: Rectangle2D, count: i32) -> RandomEntities<Line2D> {
        RandomEntities { max: count, bounds: range, generator: Random2D::random_line_int as fn(&Rectangle2D) -> Line2D, }
    }

    pub fn random_points(range: Rectangle2D, count: i32) -> RandomEntities<Point2d> {
        RandomEntities {
            max: count,
            bounds: range,
            generator: Random2D::random_point as fn(&Rectangle2D) -> Point2d,
        }
    }
}

pub struct RandomEntities<T: Sized> {
    max: i32,
    bounds: Rectangle2D,
    generator: fn(&Rectangle2D) -> T,
}

impl<T: Sized> IntoIterator for RandomEntities<T> {
    type Item = T;
    type IntoIter = RandomEntitiesIter<T>;

    fn into_iter(self) -> Self::IntoIter {
        RandomEntitiesIter {
            idx: self.max,
            bounds: self.bounds,
            generator: self.generator,
        }
    }
}

pub struct RandomEntitiesIter<T: Sized> {
    idx: i32,
    bounds: Rectangle2D,
    generator: fn(&Rectangle2D) -> T,
}

impl<T: Sized> Iterator for RandomEntitiesIter<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        if self.idx <= 0 {
            return None;
        }

        self.idx -= 1;
        Some((self.generator)(&self.bounds))
    }
}
