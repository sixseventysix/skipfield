use std::sync::atomic::{AtomicBool, Ordering};

pub struct LockLessBoolSkipfield {
    flags: Vec<AtomicBool>,
}

impl LockLessBoolSkipfield {
    pub fn new(size: usize) -> Self {
        Self {
            flags: (0..size).map(|_| AtomicBool::new(false)).collect() // initially all are skipped
        }
    }

    pub fn unskip(&self, idx: usize) -> bool {
        !self.flags[idx].swap(true, Ordering::AcqRel)
    }

    pub fn skip(&self, idx: usize) -> bool {
        self.flags[idx].swap(false, Ordering::AcqRel)
    }

    pub fn is_active(&self, idx: usize) -> bool {
        self.flags[idx].load(Ordering::Acquire)
    }

    pub fn alive_indices(&self) -> impl Iterator<Item = usize> + '_ {
        self.flags
            .iter()
            .enumerate()
            .filter(|(_, flag)| flag.load(Ordering::Acquire))
            .map(|(i, _)| i)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::thread;
    use std::sync::Arc;

    #[test]
    fn concurrent_skip_unskip_no_overlap() {
        const SIZE: usize = 1000;
        const THREADS: usize = 10;

        let skipfield = Arc::new(LockLessBoolSkipfield::new(SIZE));
        let mut handles = Vec::new();
        for t in 0..THREADS {
            let sf = Arc::clone(&skipfield);
            handles.push(thread::spawn(move || {
                let start = t * (SIZE / THREADS);
                let end = start + (SIZE / THREADS);
                for i in start..end {
                    sf.unskip(i);
                }
            }));
        }

        for handle in handles {
            handle.join().unwrap();
        }

        for i in 0..SIZE {
            assert!(skipfield.is_active(i), "Slot {} was not active!", i);
        }
    }

    #[test]
    fn test_concurrent_erase_and_check() {
        const SIZE: usize = 100000;
        const ERASE_EVERY: usize = 2;

        let skipfield = Arc::new(LockLessBoolSkipfield::new(SIZE));

        // original state of skipfield. all are unerased
        for i in 0..SIZE {
            skipfield.unskip(i);
        }

        // writer thread erasing every even indexed element starting from the first element
        let sf = Arc::clone(&skipfield);
        let eraser = thread::spawn(move || {
            for i in (0..SIZE).step_by(ERASE_EVERY) {
                sf.skip(i);
            }
        });

        // reader thread reading from the last element
        let sf2 = Arc::clone(&skipfield);
        let checker = thread::spawn(move || {
            let mut sum = 0;
            for i in (0..SIZE).rev() {
                sum += sf2.is_active(i) as usize;
            }

            // writer thread is marking exactly half of the elements as erased
            // sum is checking how many elements are unerased (i.e. in its original state).
            // this test checks if concurrent access happens on the skipfield
            // if more than half of total elements are unerased, that means our reader accessed that value before the writer got to it
            // i.e. concurrent access
            // need to use a large value for size, something small like 100 would mean the writer would blaze through marking all as erased even before the reader starts
            // this is not a hard check, this behaviour was expected and this test only exists as proving that the behaviour actually takes place
            assert!(sum > SIZE/2);
        });

        eraser.join().unwrap();
        checker.join().unwrap();

        // validate
        for i in (0..SIZE).step_by(ERASE_EVERY) {
            assert!(!skipfield.is_active(i), "Slot {} should have been erased!", i);
            assert!(skipfield.is_active(i+1), "Slot {} should still be alive!", i);
        }
    }
}
