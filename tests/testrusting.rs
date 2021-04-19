extern crate rusting;

#[test]
fn it_works() {
    assert_eq!(2 + 2, 4);
}

#[test]
fn add_two() {
    assert_eq!(4, rusting::add_two(2));
}