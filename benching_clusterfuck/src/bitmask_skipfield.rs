pub struct BitmaskSkipfield {
    chunks: Vec<u64>,
    len: usize,
}

impl BitmaskSkipfield {
    pub fn new(len: usize) -> Self {
        let num_chunks = (len+63)/64;
        Self {
            chunks: vec![0u64; num_chunks],
            len
        }
    }

    pub fn skip(&mut self, idx: usize) {
        let (chunk_idx, bit_idx) = Self::bit_pos(idx);
        self.chunks[chunk_idx] |= 1 << bit_idx;
    }

    pub fn unskip(&mut self, idx: usize) {
        let (chunk_idx, bit_idx) = Self::bit_pos(idx);
        self.chunks[chunk_idx] &= !(1 << bit_idx);
    }

    pub fn is_skipped(&self, idx: usize) -> bool {
        let (chunk_idx, bit_idx) = Self::bit_pos(idx);
        (self.chunks[chunk_idx] & (1 << bit_idx)) != 0
    }

    pub fn first_active(&self) -> Option<usize> {
        for (chunk_i, &chunk) in self.chunks.iter().enumerate() {
            let mut inv = !chunk;
            while inv != 0 {
                let tz = inv.trailing_zeros() as usize;
                let idx = chunk_i * 64 + tz;
                if idx < self.len {
                    return Some(idx);
                }
                inv &= inv - 1;
            }
        }
        None
    }

    pub fn count_skipped(&self) -> usize {
        self.chunks.iter().map(|c| c.count_ones() as usize).sum()
    }
    
    pub fn count_active(&self) -> usize {
        self.len - self.count_skipped()
    }

    pub fn active_indices_1(&self) -> impl Iterator<Item = usize> + '_ {
        (0..self.len).filter(move |&i| !self.is_skipped(i))
    }

    pub fn active_indices_2(&self) -> impl Iterator<Item = usize> + '_ {
        let len = self.len;
        self.chunks.iter().enumerate().flat_map(move |(chunk_i, &chunk)| {
            let mut inv = !chunk;
            let base = chunk_i * 64;
            std::iter::from_fn(move || {
                if inv == 0 {
                    return None;
                }
                let tz = inv.trailing_zeros() as usize;
                inv &= inv - 1; // clear the lowest set bit
                let idx = base + tz;
                if idx < len {
                    Some(idx)
                } else {
                    None
                }
            })
        })
    }

    #[inline]
    fn bit_pos(index: usize) -> (usize, usize) {
        (index / 64, index % 64)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_skip_and_unskip_behavior() {
        let mut sf = BitmaskSkipfield::new(100);
        assert_eq!(sf.count_skipped(), 0);
        assert_eq!(sf.count_active(), 100);
        assert_eq!(sf.first_active(), Some(0));

        sf.skip(0);
        sf.skip(64);
        sf.skip(99);

        assert!(sf.is_skipped(0));
        assert!(sf.is_skipped(64));
        assert!(sf.is_skipped(99));
        assert!(!sf.is_skipped(1));
        assert!(!sf.is_skipped(98));

        assert_eq!(sf.count_skipped(), 3);
        assert_eq!(sf.count_active(), 97);

        sf.unskip(64);
        assert!(!sf.is_skipped(64));
        assert_eq!(sf.count_skipped(), 2);
    }

    #[test]
    fn test_first_active_correctness() {
        let mut sf = BitmaskSkipfield::new(5);
        assert_eq!(sf.first_active(), Some(0));

        sf.skip(0);
        assert_eq!(sf.first_active(), Some(1));

        sf.skip(1);
        sf.skip(2);
        sf.skip(3);
        sf.skip(4);
        assert_eq!(sf.first_active(), None);
    }

    #[test]
    fn test_active_indices_1() {
        let mut sf = BitmaskSkipfield::new(8);
        sf.skip(2);
        sf.skip(4);
        sf.skip(7);

        let result: Vec<_> = sf.active_indices_1().collect();
        assert_eq!(result, vec![0, 1, 3, 5, 6]);
    }

    #[test]
    fn test_active_indices_2() {
        let mut sf = BitmaskSkipfield::new(70);
        sf.skip(1);
        sf.skip(5);
        sf.skip(64);
        sf.skip(69);

        let result: Vec<_> = sf.active_indices_2().collect();
        let expected: Vec<usize> = (0..70).filter(|&i| i != 1 && i != 5 && i != 64 && i != 69).collect();
        assert_eq!(result, expected);
    }

    #[test]
    fn test_chunk_boundary_behavior() {
        let mut sf = BitmaskSkipfield::new(128);
        sf.skip(63);
        sf.skip(64);
        sf.skip(65);

        assert!(sf.is_skipped(63));
        assert!(sf.is_skipped(64));
        assert!(sf.is_skipped(65));
        assert_eq!(sf.count_skipped(), 3);

        let act1: Vec<_> = sf.active_indices_1().collect();
        let act2: Vec<_> = sf.active_indices_2().collect();
        assert_eq!(act1, act2);
    }

    #[test]
    fn test_empty_skipfield() {
        let sf = BitmaskSkipfield::new(0);
        assert_eq!(sf.count_skipped(), 0);
        assert_eq!(sf.count_active(), 0);
        assert_eq!(sf.first_active(), None);
        assert_eq!(sf.active_indices_1().count(), 0);
        assert_eq!(sf.active_indices_2().count(), 0);
    }
}