use std::fmt;

use crate::entities::point2d::Point2d;

#[derive(Copy, Clone, Debug)]
pub struct Extents2d {
    pub min: Point2d,
    pub max: Point2d,
}

impl fmt::Display for Extents2d {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "(Min: {}, Max: {})", self.min, self.max)
    }
}

impl Extents2d
{
    pub fn new_square(size: f32) -> Self
    {
        let min: Point2d = Point2d::origin();
        let max: Point2d = Point2d::new(size, size);
        
        Extents2d
        {
            min,
            max
        }
    }
}
