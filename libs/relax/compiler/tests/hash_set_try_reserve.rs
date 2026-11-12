// `HashSet::try_reserve` (stable since 1.57), as turso_core's fallible `TryClone` for HashSet calls it.
use std::collections::HashSet;

#[test]
fn try_reserve_then_insert() {
    let mut s: HashSet<u32> = HashSet::new();
    assert!(s.try_reserve(100).is_ok());
    assert!(s.capacity() >= 100);
    for i in 0..100 {
        s.insert(i);
    }
    assert_eq!(s.len(), 100);
}

#[test]
fn try_reserve_keeps_contents() {
    let mut s: HashSet<&str> = ["a", "b"].into_iter().collect();
    s.try_reserve(10).unwrap();
    assert!(s.contains("a") && s.contains("b"));
    assert_eq!(s.len(), 2);
}
