pub struct PermutationIter {
    i: usize,
    stack: Vec<usize>,
}

impl PermutationIter {
    pub fn new(max: usize) -> Self {
        if max == usize::MAX {
            panic!("Max must be larger than usize max");
        }

        let mut stack: Vec<usize> = Vec::with_capacity(max);

        for i in 0..max {
            stack[i] = 0;
        }

        PermutationIter { i: max, stack }
    }
}

impl Iterator for PermutationIter {
    type Item = usize;

    fn next(&mut self) -> Option<usize> {
        if self.i >= self.stack.len() {
            return None;
        }

        while self.i < self.stack.len() {
            if self.stack[self.i] < self.i {
                if self.stack[self.i] % 2 == 0 {
                    self.stack.swap(0, self.stack[self.i]);
                } else {
                    self.stack.swap()
                }
            } else {
                self.stack[self.i] = 0;
                self.i += 1;
            }
        }

        None
    }
}
