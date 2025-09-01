use crate::entities::{line2d::Line2D, point2d::Point2d};
use std::{cmp::Ordering, ops::{Deref, DerefMut}};

//Wrapper around Point2D struct for lexicographic comparisons
#[repr(transparent)]
#[derive(Debug, Clone, Copy)]
pub struct LexicographicPoint2d(pub Point2d);

impl LexicographicPoint2d {
    pub fn new(x: f32, y: f32) -> Self
    {
        Self(Point2d { x, y })
    }
}

impl PartialEq for LexicographicPoint2d
{
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}

impl From<Point2d> for LexicographicPoint2d
{
    fn from(value: Point2d) -> Self {
        Self(value)
    }
}

impl PartialOrd for LexicographicPoint2d {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        if let Some(c) = self.0.y.partial_cmp(&other.0.y)
        {
            match c
            {
                std::cmp::Ordering::Equal => {return self.0.x.partial_cmp(&other.0.x);},
                _ => {return Some(c);}
            }
        }
        
        None
    }
}

// impl Ord for LexicographicPoint2d {
//     fn cmp(&self, other: &Self) -> std::cmp::Ordering {
//         self.0.y.total_cmp(&other.0.y)
//         .then(self.0.x.total_cmp(&other.0.x))
//     }
// }


impl Deref for LexicographicPoint2d {
    type Target = Point2d;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for LexicographicPoint2d {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy)]
pub struct LexicographicLine2d(pub Line2D);

impl PartialEq for LexicographicLine2d {
    fn eq(&self, other: &Self) -> bool {
        LexicographicPoint2d(self.0.start) == LexicographicPoint2d(other.0.start) 
        && 
        LexicographicPoint2d(self.0.end) == LexicographicPoint2d(other.0.end)
    }
}

impl PartialOrd for LexicographicLine2d
{
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        if let Some(comp) = LexicographicPoint2d(self.0.start)
        .partial_cmp(&LexicographicPoint2d(other.0.start))
        {
            if comp != Ordering::Equal
            {
                return LexicographicPoint2d(self.0.end)
                .partial_cmp(&LexicographicPoint2d(other.0.end))
            }
            else {
                return Some(comp);
            }

        }
        else {
            return None;
        }
    }
}

impl From<Line2D> for LexicographicLine2d
{
    fn from(value: Line2D) -> Self {
        Self(value)
    }
}

impl LexicographicLine2d
{
    //Create a new Lexicographic line, normalized so that start < end
    pub fn new_normalized(line: Line2D) -> Self
    {
        if LexicographicPoint2d(line.start) < LexicographicPoint2d(line.end)
        {
            return LexicographicLine2d(line);
        }
        else
        {
            return LexicographicLine2d(line.reverse());
        }
    }

    pub fn upper_lower(upper: LexicographicPoint2d, lower: LexicographicPoint2d) -> Self
    {
        LexicographicLine2d(Line2D { start: upper.0, end: lower.0 })
    }

    pub fn line(&self) -> Line2D
    {
        self.0
    }

    pub fn start(&self) -> LexicographicPoint2d
    {
        LexicographicPoint2d(self.0.start)
    }

    pub fn end(&self) -> LexicographicPoint2d
    {
        LexicographicPoint2d(self.0.end)
    }
}