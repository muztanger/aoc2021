//use std::fs;
//use itertools::Itertools;

pub fn moop() -> usize {
    return 3
}

#[test]
fn test_moop() {
    assert_eq!(moop(), 3);
}

#[test]
fn test_meep() {
    assert_ne!(moop(), 3);
}