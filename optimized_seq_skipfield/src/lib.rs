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
            
