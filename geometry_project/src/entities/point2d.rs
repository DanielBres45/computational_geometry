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
