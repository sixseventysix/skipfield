use lockless_boolean_skipfield::LockLessBoolSkipfield;

fn main() {
    let sf = LockLessBoolSkipfield::new(10);
    sf.unskip(2);
    sf.unskip(5);

    assert_eq!(sf.is_active(2), true);
    assert_eq!(sf.is_active(3), false);

    sf.skip(2);
    assert_eq!(sf.is_active(2), false);

    for i in sf.alive_indices() {
        println!("Slot {} is alive", i);
    }
}

