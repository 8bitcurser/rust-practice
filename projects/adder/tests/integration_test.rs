// we need this as each test in the test
// directory is a separate crate, so we
// need the lib at the crates scope.
use adder;

#[test]
fn it_adds_two_the_combeack() {
    assert_eq!(4, adder::add_two(2));
    println!("YEY");
}
