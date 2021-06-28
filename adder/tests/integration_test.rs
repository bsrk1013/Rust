extern crate adder;

mod common;

#[test]
fn it_add_two() {
    common::setup();
    let data = 5;
    assert_eq!(adder::add_two(data), data + 2);
}
