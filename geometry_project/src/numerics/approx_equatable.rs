use super::floating_comparisons;

pub trait ApproxEquals<Rhs = Self>
where
    Rhs: ?Sized,
{
    fn approx_equals(&self, other: &Rhs, epsilon: f32) -> bool;
}

impl ApproxEquals<f32> for f32 {
    fn approx_equals(&self, other: &f32, epsilon: f32) -> bool {
        floating_comparisons::approx_equal(self.clone(), other.clone(), epsilon)
    }
}
