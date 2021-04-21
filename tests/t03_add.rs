pub fn add_three(a: i32) -> i32 {
    a + 3
}

#[test]
fn test_add_three() {
    assert_eq!(5, add_three(2));
}
