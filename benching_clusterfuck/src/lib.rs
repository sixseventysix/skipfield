pub struct Skipfield {
    chunks: Vec<u64>,
    len: usize,
}

impl Skipfield {
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
        self.chunks[chunk_idx] &= 1 << bit_idx;
    }

    pub fn is_skipped(&self, index: usize) -> bool {
        let (chunk_idx, bit_idx) = Self::bit_pos(index);
        (self.chunks[chunk_idx] & (1 << bit_idx)) == 1
    }

    pub fn first_free(&self) -> Option<usize> {
        for (chunk_i, &chunk) in self.chunks.iter().enumerate() {
            let inv = !chunk;
            if inv != 0 {
                return Some(chunk_i * 64 + inv.trailing_zeros() as usize);
            }
        }
        None
    }

    pub fn count_skipped(&self) -> usize {
        let total_bits = self.len;
        let skipped = self.chunks.iter().map(|c| c.count_ones() as usize).sum::<usize>();
        total_bits - skipped
    }

    #[inline]
    fn bit_pos(index: usize) -> (usize, usize) {
        (index / 64, index % 64)
    }
}
            
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
