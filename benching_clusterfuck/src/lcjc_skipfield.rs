pub struct LCJCSkipfield {
    nodes: Vec<u8>,
}

impl LCJCSkipfield {
    pub fn new(size: usize) -> Self {
        Self { nodes: vec![0; size] }
    }

    pub fn skip(&mut self, i: usize) {
        let left = if i > 0 { self.nodes[i - 1] } else { 0 };
        let right = if i + 1 < self.nodes.len() { self.nodes[i + 1] } else { 0 };

        match (left, right) {
            (0, 0) => {
                self.nodes[i] = 1;
            }
            (0, r) => {
                let end = i + r as usize;
                let val = r + 1;
                self.nodes[end] = val;
                self.nodes[i] = val;
            }
            (l, 0) => {
                let start = i - l as usize;
                let val = l + 1;
                self.nodes[start] = val;
                self.nodes[i] = val;
            }
            (l, r) => {
                let start = i - l as usize;
                let end = i + r as usize;
                let val = l + r + 1;
                self.nodes[start] = val;
                self.nodes[end] = val;
            }
        }
    }

    pub fn unskip(&mut self, i: usize, start: Option<usize>, end: Option<usize>) {
        if self.nodes[i] == 0 {
            return;
        }

        match (start, end) {
            (Some(s), Some(e)) if i > s && i < e => {
                let left_len = i - s;
                let right_len = e - i;
                self.nodes[s] = left_len as u8;
                self.nodes[i - 1] = left_len as u8;
                self.nodes[e] = right_len as u8;
                self.nodes[i + 1] = right_len as u8;
                self.nodes[i] = 0;
            }
            (Some(s), _) if i == s => {
                let len = self.nodes[s] - 1;
                let e = i + len as usize;
                self.nodes[i + 1] = len;
                self.nodes[e] = len;
                self.nodes[i] = 0;
            }
            (_, Some(e)) if i == e => {
                let len = self.nodes[e] - 1;
                let s = i - len as usize;
                self.nodes[i - 1] = len;
                self.nodes[s] = len;
                self.nodes[i] = 0;
            }
            _ => {
                self.nodes[i] = 0;
            }
        }
    }

    pub fn is_skipped(&self, i: usize) -> bool {
        self.nodes[i] != 0
    }

    pub fn count_skipped(&self) -> usize {
        let mut i = 0;
        let mut total = 0;
        while i < self.nodes.len() {
            if self.nodes[i] == 0 {
                i += 1;
            } else {
                let skip = self.nodes[i] as usize;
                total += skip;
                i += skip;
            }
        }
        total
    }

    pub fn active_indices(&self) -> impl Iterator<Item = usize> + '_ {
        let mut i = 0;
        std::iter::from_fn(move || {
            while i < self.nodes.len() {
                if self.nodes[i] == 0 {
                    let out = Some(i);
                    i += 1;
                    return out;
                } else {
                    i += self.nodes[i] as usize;
                }
            }
            None
        })
    }
    
    pub fn count_active(&self) -> usize {
        self.nodes.len() - self.count_skipped()
    }

    pub fn first_active(&self) -> Option<usize> {
        let mut i = 0;
        while i < self.nodes.len() {
            if self.nodes[i] == 0 {
                return Some(i);
            } else {
                i += self.nodes[i] as usize;
            }
        }
        None
    }

    pub fn debug(&self) -> &[u8] {
        &self.nodes
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_skip_and_counting() {
        let mut sf = LCJCSkipfield::new(10);
        assert_eq!(sf.count_skipped(), 0);
        assert_eq!(sf.count_active(), 10);
        assert_eq!(sf.first_active(), Some(0));

        sf.skip(2);
        assert!(sf.is_skipped(2));
        assert_eq!(sf.count_skipped(), 1);
        assert_eq!(sf.count_active(), 9);

        sf.skip(3);
        sf.skip(4);
        assert!(sf.is_skipped(2));
        assert!(sf.is_skipped(3));
        assert!(sf.is_skipped(4));
        assert_eq!(sf.count_skipped(), 3);
        assert_eq!(sf.count_active(), 7);
    }

    #[test]
    fn test_unskip_middle_node_in_block() {
        let mut sf = LCJCSkipfield::new(10);
        sf.skip(1);
        sf.skip(2);
        sf.skip(3);

        sf.unskip(2, Some(1), Some(3));
        assert!(!sf.is_skipped(2));
        assert!(sf.is_skipped(1));
        assert!(sf.is_skipped(3));
        assert_eq!(sf.count_skipped(), 2);
        assert_eq!(sf.count_active(), 8);
    }

    #[test]
    fn test_unskip_block_start_and_end() {
        let mut sf = LCJCSkipfield::new(10);
        sf.skip(4);
        sf.skip(5);
        sf.skip(6);

        sf.unskip(4, Some(4), None);
        assert!(!sf.is_skipped(4));
        assert!(sf.is_skipped(5));
        assert!(sf.is_skipped(6));

        sf.unskip(6, None, Some(6));
        assert!(!sf.is_skipped(6));
        assert!(sf.is_skipped(5));

        sf.unskip(5, Some(5), Some(5));
        assert!(!sf.is_skipped(5));
        assert_eq!(sf.count_skipped(), 0);
        assert_eq!(sf.count_active(), 10);
    }

    #[test]
    fn test_first_active_edge_cases() {
        let mut sf = LCJCSkipfield::new(4);
        assert_eq!(sf.first_active(), Some(0));

        sf.skip(0);
        assert_eq!(sf.first_active(), Some(1));

        sf.skip(1);
        sf.skip(2);
        sf.skip(3);
        assert_eq!(sf.first_active(), None);
    }

    #[test]
    fn test_active_indices_correctness() {
        let mut sf = LCJCSkipfield::new(8);
        sf.skip(1);
        sf.skip(2);
        sf.skip(5);

        let active: Vec<_> = sf.active_indices().collect();
        assert_eq!(active, vec![0, 3, 4, 6, 7]);
    }

    #[test]
    fn test_debug_state_shape() {
        let mut sf = LCJCSkipfield::new(6);
        sf.skip(2);
        sf.skip(3);
        sf.skip(4);

        let state = sf.debug();
        assert_eq!(state.len(), 6);
        assert_eq!(state[0], 0);
        assert_eq!(state[1], 0);
        assert!(state[2] > 0);
        assert_eq!(state[2], state[4]);
        assert_eq!(state[5], 0);
    }
}