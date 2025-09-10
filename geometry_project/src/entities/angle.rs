use std::{f64::consts::PI, ops::{Add, AddAssign}};


#[derive(Copy, Clone, Debug)]
pub struct Angle(pub f64);

pub const RADIANS_TO_DEGREES: f64 = 180f64 / PI;
pub const DEGREES_TO_RADIANS: f64 = PI / 180f64;

impl Add<Angle> for Angle
{
    type Output = Angle;

    fn add(self, rhs: Angle) -> Self::Output {
        Angle(self.0 + rhs.0)
    }
}

impl Add<&Angle> for &Angle {
    type Output = Angle;

    fn add(self, rhs: &Angle) -> Self::Output {
        Angle(self.0 + rhs.0)
    }
}

impl AddAssign<Angle> for Angle {
    fn add_assign(&mut self, rhs: Angle) {
        self.0 += rhs.0;
    }
}

impl Angle
{
    pub fn radians(&self) -> f64
    {
        self.0
    }

    pub fn from_degrees(degrees: f64) -> Self
    {
        Angle(degrees * DEGREES_TO_RADIANS)
    }

    pub fn clamp(&self, min: f64, max: f64) -> Angle {
        Angle(self.0.clamp(min, max))
    }

    pub fn degrees(&self) -> f64
    {
        self.0 * RADIANS_TO_DEGREES
    }

    pub fn modulo(&self, other: &Angle) -> Angle
    {
        Angle(self.0.rem_euclid(other.0))
    }
}

