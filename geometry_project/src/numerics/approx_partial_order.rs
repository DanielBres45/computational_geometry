use std::{cmp::Ordering, f32::EPSILON};

use super::approx_equatable::ApproxEquals;

pub trait ApproxPartialOrder<Rhs = Self>: ApproxEquals<Rhs>
where
    Self: PartialOrd<Rhs>,
    Rhs: ?Sized + PartialOrd<Rhs> + PartialEq<Rhs>,
{
    fn approx_partial_order(&self, other: &Rhs, epsilon: f32) -> Option<Ordering> {
        if self.approx_equals(other, epsilon) {
            return Some(Ordering::Equal);
        }

        PartialOrd::partial_cmp(self, other)
    }

    fn approx_less(&self, other: &Rhs, epsilon: f32) -> bool {
        self.approx_partial_order(other, epsilon)
            .is_some_and(Ordering::is_lt)
    }

    fn approx_equals_or_less_than(&self, other: &Rhs, epsilon: f32) -> bool {
        self.approx_partial_order(other, epsilon)
            .is_some_and(|o| o.is_eq() || o.is_lt())
    }

    fn approx_greater(&self, other: &Rhs, epsilon: f32) -> bool {
        self.approx_partial_order(other, epsilon)
            .is_some_and(Ordering::is_gt)
    }

    fn approx_equals_or_greater_than(&self, other: &Rhs, epsilon: f32) -> bool {
        self.approx_partial_order(other, epsilon)
            .is_some_and(|o| o.is_eq() || o.is_gt())
    }
}

impl ApproxPartialOrder for f32 {}
