pub struct BitmaskSkipfield {
    chunks: Vec<u64>,
    len: usize,
}

impl BitmaskSkipfield {
    pub fn new(len: usize) -> Self {
        let num_chunks = (len + 63) / 64;
        let mut chunks = vec![0u64; num_chunks];
    
        let extra_bits = (num_chunks * 64).saturating_sub(len);
        if extra_bits > 0 {
            let mask = u64::MAX << (64 - extra_bits);
            chunks[num_chunks - 1] |= mask;
        }
    
        Self { chunks, len }
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
            let inv = !chunk;
            if inv != 0 {
                return Some(chunk_i * 64 + inv.trailing_zeros() as usize);
            }
        }
        None
    }

    pub fn count_skipped(&self) -> usize {
        let full_chunks = self.len / 64;
        let tail_bits = self.len % 64;
    
        let mut count = self.chunks[..full_chunks]
            .iter()
            .map(|c| c.count_ones() as usize)
            .sum::<usize>();
    
        if tail_bits > 0 {
            let last_chunk = self.chunks[full_chunks];
            let mask = (1u64 << tail_bits) - 1;
            count += (last_chunk & mask).count_ones() as usize;
        }
    
        count
    }
    
    pub fn count_active(&self) -> usize {
        self.len - self.count_skipped()
    }

    pub fn active_indices_1(&self) -> impl Iterator<Item = usize> + '_ {
        (0..self.len).filter(move |&i| !self.is_skipped(i))
    }

    pub fn active_indices_2(&self) -> impl Iterator<Item = usize> + '_ {
        self.chunks.iter().enumerate().flat_map(move |(chunk_i, &chunk)| {
            let mut inv = !chunk;
            let base = chunk_i * 64;
            std::iter::from_fn(move || {
                if inv == 0 {
                    return None;
                }
                let tz = inv.trailing_zeros() as usize;
                inv &= inv - 1;
                Some(base + tz)
            })
        })
    }

    #[inline]
    fn bit_pos(index: usize) -> (usize, usize) {
        (index / 64, index % 64)
    }

    pub fn iter(&self) -> BitmaskSkipfieldIter<'_> {
        BitmaskSkipfieldIter::new(&self.chunks, self.len)
    }
}

pub struct BitmaskSkipfieldIter<'a> {
    chunks: &'a [u64],
    len: usize,
    chunk_i: usize,
    bitset: u64,
}

impl<'a> BitmaskSkipfieldIter<'a> {
    pub fn new(chunks: &'a [u64], len: usize) -> Self {
        let mut iter = Self {
            chunks,
            len,
            chunk_i: 0,
            bitset: 0,
        };
        if !chunks.is_empty() {
            iter.bitset = !chunks[0];
        }
        iter
    }

    fn advance_to_next_chunk(&mut self) {
        while self.chunk_i < self.chunks.len() {
            let inv = !self.chunks[self.chunk_i];
            if inv != 0 {
                self.bitset = inv;
                return;
            }
            self.chunk_i += 1;
        }
    }
}

impl<'a> Iterator for BitmaskSkipfieldIter<'a> {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            while self.bitset != 0 {
                let tz = self.bitset.trailing_zeros() as usize;
                self.bitset &= self.bitset - 1;
                return Some(self.chunk_i * 64 + tz);
            }
    
            self.chunk_i += 1;
            if self.chunk_i >= self.chunks.len() {
                return None;
            }
    
            let inv = !self.chunks[self.chunk_i];
            if inv != 0 {
                self.bitset = inv;
            }
        }
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

    #[test]
    fn test_no_active_indices_beyond_len() {
        let len = 70;
        let num_chunks = (len + 63) / 64;
        let total_bits = num_chunks * 64;
        let extra_bits_start = len;
        let extra_bits_end = total_bits;

        let sf = BitmaskSkipfield::new(len);

        for i in extra_bits_start..extra_bits_end {
            let (chunk_idx, bit_idx) = BitmaskSkipfield::bit_pos(i);
            assert!(
                (sf.chunks[chunk_idx] & (1 << bit_idx)) != 0,
                "Extra bit at index {} should be set (skipped)", i
            );
        }
    }

    #[test]
    fn test_first_active_never_exceeds_len() {
        let len = 70;
        let sf = BitmaskSkipfield::new(len);

        if let Some(i) = sf.first_active() {
            assert!(
                i < len,
                "first_active returned index {} which is out of bounds (len = {})",
                i,
                len
            );
        }
    }

    #[test]
    fn test_active_indices_2_with_tail_bits() {
        let len = 70;
        let mut sf = BitmaskSkipfield::new(len);
        sf.skip(0);
        sf.skip(69);

        let indices: Vec<_> = sf.active_indices_2().collect();

        assert!(!indices.contains(&0));
        assert!(!indices.contains(&69));
        assert!(indices.iter().all(|&i| i < len));
    }

    #[test]
    fn test_all_bits_skipped_if_len_zero() {
        let sf = BitmaskSkipfield::new(0);
        assert_eq!(sf.first_active(), None);
        assert_eq!(sf.count_active(), 0);
        assert_eq!(sf.count_skipped(), 0);
        assert_eq!(sf.active_indices_2().count(), 0);
    }

    #[test]
    fn test_single_active_bit() {
        let mut skipfield = BitmaskSkipfield::new(64);
        for i in 0..64 {
            skipfield.skip(i);
        }
        skipfield.unskip(7); // mark index 7 as active

        let mut iter = skipfield.iter().take(2);
        assert_eq!(iter.next(), Some(7));
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn test_multiple_active_indices() {
        let mut skipfield = BitmaskSkipfield::new(128);
        for i in 0..128 {
            skipfield.skip(i);
        }

        let active = [3, 66, 100];
        for &i in &active {
            skipfield.unskip(i);
        }

        let collected: Vec<_> = skipfield.iter().collect();
        assert_eq!(collected, active);
    }

    #[test]
    fn test_no_active_bits() {
        let mut skipfield = BitmaskSkipfield::new(128);
        for i in 0..128 {
            skipfield.skip(i);
        }

        let mut iter = skipfield.iter();
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn test_all_active_bits() {
        let skipfield = BitmaskSkipfield::new(128);
        let expected: Vec<_> = (0..128).collect();
        let collected: Vec<_> = skipfield.iter().collect();
        assert_eq!(collected, expected);
    }

    #[test]
    fn test_bounds_handling_on_last_chunk() {
        let len = 70;
        let mut skipfield = BitmaskSkipfield::new(len);
        for i in 0..len {
            if i != 69 {
                skipfield.skip(i);
            }
        }

        let result = skipfield.iter().next();
        assert_eq!(result, Some(69));
    }
}