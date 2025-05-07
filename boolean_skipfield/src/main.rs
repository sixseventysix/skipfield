fn main() {
    let mut sf = boolean_skipfield::BoolSkipfield::new(10);
    sf.skip(1);
    sf.skip(3);
    sf.skip(7);

    println!("Skipped count: {}", sf.count_skipped());
    println!("Active count: {}", sf.count_active());
    println!("First active: {:?}", sf.first_active());
}
