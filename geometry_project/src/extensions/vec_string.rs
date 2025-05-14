use std::fmt::{Debug, Display};

pub struct VecToStringIter<'a, T>
where
    T: Debug + Display,
{
    items: &'a Vec<T>,
    idx: usize,
}
