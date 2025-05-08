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

    pub fn active_indices(&self) -> impl Iterator<Item = usize> + '_ {
        self.flags
            .iter()
            .enumerate()
            .filter_map(|(i, &bit)| if !bit { Some(i) } else { None })
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_skip_and_unskip() {
        let mut sf = BoolSkipfield::new(5);

        assert_eq!(sf.count_skipped(), 0);
        assert_eq!(sf.count_active(), 5);

        sf.skip(1);
        sf.skip(3);
        assert!(sf.is_skipped(1));
        assert!(sf.is_skipped(3));
        assert!(!sf.is_skipped(0));
        assert_eq!(sf.count_skipped(), 2);
        assert_eq!(sf.count_active(), 3);

        sf.unskip(1);
        assert!(!sf.is_skipped(1));
        assert_eq!(sf.count_skipped(), 1);
        assert_eq!(sf.count_active(), 4);
    }

    #[test]
    fn test_first_active() {
        let mut sf = BoolSkipfield::new(4);
        assert_eq!(sf.first_active(), Some(0));

        sf.skip(0);
        assert_eq!(sf.first_active(), Some(1));

        sf.skip(1);
        sf.skip(2);
        sf.skip(3);
        assert_eq!(sf.first_active(), None);
    }

    #[test]
    fn test_active_indices() {
        let mut sf = BoolSkipfield::new(6);
        sf.skip(1);
        sf.skip(4);

        let active: Vec<_> = sf.active_indices().collect();
        assert_eq!(active, vec![0, 2, 3, 5]);
    }

    #[test]
    fn test_empty_skipfield() {
        let sf = BoolSkipfield::new(0);
        assert_eq!(sf.count_skipped(), 0);
        assert_eq!(sf.count_active(), 0);
        assert_eq!(sf.first_active(), None);
        assert_eq!(sf.active_indices().count(), 0);
    }

    #[test]
    fn test_full_skip() {
        let mut sf = BoolSkipfield::new(3);
        sf.skip(0);
        sf.skip(1);
        sf.skip(2);

        assert_eq!(sf.count_skipped(), 3);
        assert_eq!(sf.count_active(), 0);
        assert_eq!(sf.first_active(), None);
        let indices: Vec<_> = sf.active_indices().collect();
        assert!(indices.is_empty());
    }
}
