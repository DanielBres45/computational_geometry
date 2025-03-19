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

#[cfg(test)]
mod tests {
    use crate::extensions::vec_extensions::VecExtensions;

    #[test]
    pub fn test_from_last() {
        let items: Vec<i32> = [0, 1, 2, 3, 4].into();

        assert_eq!(4, items.from_last(0));
        assert_eq!(3, items.from_last(1));
        assert_eq!(2, items.from_last(2));
        assert_eq!(1, items.from_last(3));
    }
}
