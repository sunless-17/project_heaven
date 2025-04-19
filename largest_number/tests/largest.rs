use largest_number::large_number::largest_number;

// supposed to pass
#[test]
fn test_largest() {
    assert_eq!(largest_number(1, 2), 2);
    assert_eq!(largest_number(2, 1), 2);
}

// supposed to fail
#[test]
fn test_largest_failure() {
    assert_ne!(largest_number(1, 2), 2);
    assert_ne!(largest_number(2, 1), 2);
}
