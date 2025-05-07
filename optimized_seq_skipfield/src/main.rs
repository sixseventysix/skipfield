fn main() {
   let mut sf = optimized_seq_skipfield::Skipfield::new(100_000);

    for i in (0..50_000) {
        sf.skip(i);
    }

    println!("First free: {:?}", sf.first_free());
    println!("Alive count: {}", sf.count_skipped());
    println!("Is idx 124 skipped: {}", sf.is_skipped(124));
}
