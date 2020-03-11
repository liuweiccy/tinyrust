use std::fmt::Debug;
use std::ops::Mul;

fn foo<T>(x : T) -> T {
    x
}

#[test]
fn test_generic1() {
    assert_eq!(foo(1), 1);
}

