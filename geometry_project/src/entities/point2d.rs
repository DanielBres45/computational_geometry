use std::fmt;

#[derive(Copy, Clone, Debug)]
pub struct Point2d {
    pub x: f32,
    pub y: f32,
}

impl fmt::Display for Point2d {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

impl Point2d
{
    pub fn origin() -> Self
    {
        Point2d { x: 0f32, y: 0f32 }
    }

    pub fn new(x: f32, y: f32) -> Self
    {
        Point2d{x, y}
    }
}
