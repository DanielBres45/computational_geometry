pub struct MixedIncremenet {
    bases: Vec<usize>,
    indexes: Vec<usize>,
    done: bool,
}

impl MixedIncremenet {
    pub fn new(bases: Vec<usize>) -> Self {
        let indexes: Vec<usize> = vec![0; bases.len()];

        MixedIncremenet {
            bases,
            indexes,
            done: false,
        }
    }

    pub fn new_uniform(base: usize, length: usize) -> Self {
        let bases: Vec<usize> = vec![base; length];
        let indexes: Vec<usize> = vec![0; length];

        MixedIncremenet {
            bases,
            indexes,
            done: false,
        }
    }
}

impl Iterator for MixedIncremenet {
    type Item = Vec<usize>;

    fn next(&mut self) -> Option<Vec<usize>> {
        if self.done {
            return None;
        }

        let items = self.indexes.clone();

        for i in (0..self.indexes.len()).rev() {
            self.indexes[i] += 1;

            if self.indexes[i] < self.bases[i] {
                break;
            }

            self.indexes[i] = 0;

            if i == 0 {
                self.done = true;
            }
        }

        return Some(items);
    }
}
