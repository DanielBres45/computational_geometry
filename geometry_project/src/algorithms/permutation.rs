use std::fmt::{Debug, Display};

#[derive(Debug)]
pub struct PermutationIter {
    i: usize,
    items: Vec<i32>,
    stack: Vec<usize>,
}

impl Display for PermutationIter {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "PermutationIter [i:{}, items:{}, stack:{} ]",
            self.i,
            itertools::join(&self.items, ", "),
            itertools::join(&self.stack, ",")
        )
    }
}

impl PermutationIter {
    pub fn new(count: usize) -> Self {
        let mut stack: Vec<usize> = Vec::with_capacity(count);
        let mut items: Vec<i32> = Vec::with_capacity(count);

        for i in 0..count {
            stack.push(0);
            items.push(i as i32);
        }

        PermutationIter { i: 0, items, stack }
    }
}

impl Iterator for PermutationIter {
    type Item = Vec<i32>;

    fn next(&mut self) -> Option<Vec<i32>> {
        if self.i == 0 {
            self.i += 1;
            return Some(self.items.clone());
        }

        while self.i < self.stack.len() {
            if self.stack[self.i] < self.i {
                if self.i % 2 == 0 {
                    self.items.swap(0, self.i);
                } else {
                    self.items.swap(self.stack[self.i], self.i);
                }

                self.stack[self.i] += 1;
                self.i = 1;

                return Some(self.items.clone());
            } else {
                self.stack[self.i] = 0;
                self.i += 1;
            }
        }

        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::testing_tools::assert_helper::AssertHelper;

    #[test]
    fn test_permute_1() {
        let mut iter = PermutationIter::new(1);

        let expected: [Option<Vec<i32>>; 2] = [Some(vec![0]), None];

        for i in 0..2 {
            let found = iter.next();
            AssertHelper::option_equal_vec(expected[i].clone(), found);
        }
    }

    #[test]
    fn test_permute_2() {
        let mut iter = PermutationIter::new(2);

        let expected: [Option<Vec<i32>>; 3] = [Some(vec![0, 1]), Some(vec![1, 0]), None];

        for i in 0..3 {
            let found = iter.next();
            AssertHelper::option_equal_vec(expected[i].clone(), found);
        }
    }

    #[test]
    fn test_permute_3() {
        let mut iter = PermutationIter::new(3);

        let expected: [Option<Vec<i32>>; 7] = [
            Some(vec![0, 1, 2]),
            Some(vec![1, 0, 2]),
            Some(vec![2, 0, 1]),
            Some(vec![0, 2, 1]),
            Some(vec![1, 2, 0]),
            Some(vec![2, 1, 0]),
            None,
        ];

        for i in 0..7 {
            let found = iter.next();
            println!("{}", &iter);
            AssertHelper::option_equal_vec(expected[i].clone(), found);
        }
    }
}
