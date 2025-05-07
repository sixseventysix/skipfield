pub struct BoolSkipfield {
    flags: Vec<bool>,
}

impl BoolSkipfield {
    pub fn new(size: usize) -> Self {
        Self {
            flags: vec![false; size],
        }
    }

    pub fn skip(&mut self, index: usize) {
        self.flags[index] = true;
    }

    pub fn unskip(&mut self, index: usize) {
        self.flags[index] = false;
    }

    pub fn is_skipped(&self, index: usize) -> bool {
        self.flags[index]
    }

    pub fn count_skipped(&self) -> usize {
        let mut count = 0;
        for &bit in &self.flags {
            if bit {
                count += 1;
            }
        }
        count
    }

    pub fn count_active(&self) -> usize {
        self.flags.len() - self.count_skipped()
    }

    pub fn first_active(&self) -> Option<usize> {
        for (i, &bit) in self.flags.iter().enumerate() {
            if !bit {
                return Some(i);
            }
        }
        None
    }
}
