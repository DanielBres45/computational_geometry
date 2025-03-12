use crate::entities::point2d::Point2d;
use std::cmp::Ordering;

pub trait VecExtensions<T> {
    fn from_last(&self, index: usize) -> T;
}

impl<T: Copy> VecExtensions<T> for Vec<T> {
    fn from_last(&self, index: usize) -> T {
        if index >= self.len() {
            panic!("Index out of range. {} - {} < 0", index, self.len());
        }

        self[self.len() - 1 - index]
    }
}

pub trait VecPointExtesions {
    fn sort_lexicographic(&mut self);
}

impl VecPointExtesions for Vec<Point2d> {
    fn sort_lexicographic(&mut self) {
        self.sort_by(|a, b| -> Ordering {
            let x_cmpr = a.x.total_cmp(&b.x);

            if !matches!(x_cmpr, Ordering::Equal) {
                return x_cmpr;
            }

            return a.y.total_cmp(&b.y);
        }); //lexigraphical Ordering
    }
}
