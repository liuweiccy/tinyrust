use std::fmt::Debug;
use std::ops::Mul;

fn foo<T>(x: T) -> T {
    x
}

#[test]
fn test_generic1() {
    assert_eq!(foo(1), 1);
    assert_eq!(foo("hello"), "hello");
}

#[derive(Debug, PartialEq)]
struct Point<T> {
    x: T,
    y: T,
}

impl<T> Point<T> {
    fn new(x: T, y: T) -> Self {
        Point { x, y }
    }
}

#[test]
fn test_generic2() {
    let point1 = Point::new(1, 2);
    let point2 = Point::new("1", "2");

    assert_eq!(point1, Point { x: 1, y: 2 });
    assert_eq!(point2, Point { x: "1", y: "2" });
}

fn foo_1(x: i32) -> i32 {
    x
}

fn foo_2(x: &'static str) -> &'static str {
    x
}

#[test]
fn test_generic3() {
    foo_1(1);
    foo_2("2");
}

#[derive(Debug, PartialEq)]
struct Foo(i32);
#[derive(Debug, PartialEq)]
struct Bar(i32, i32);

trait Inst {
    fn new(i: i32) -> Self;
}

impl Inst for Foo {
    fn new(i: i32) -> Self {
        Foo(i)
    }
}

impl Inst for Bar {
    fn new(i: i32) -> Self {
        Bar(i, i + 10)
    }
}

// 静态分发
fn foobar<T: Inst>(i: i32) -> T {
    T::new(i)
}

#[test]
fn test_generic_static_distribute() {
    let f: Foo = foobar(3);
    assert_eq!(f, Foo(3));

    let b: Bar = foobar(3);
    assert_eq!(b, Bar(3, 13));
}
