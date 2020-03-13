use std::ops::Add;

trait Page {
    fn set_page(&self, p: i32) {
        println!("Page Default:1");
    }
}

trait PerPage {
    fn set_perpage(&self, num: i32) {
        println!("Per Page Default: 10");
    }
}

trait Paginate: Page + PerPage {
    fn set_skip_page(&self, num: i32) {
        println!("Skip Page: {:?}", num);
    }
}

impl<T: Page + PerPage> Paginate for T {}

struct MyPaginate {
    page: i32,
}
impl Page for MyPaginate {}
impl PerPage for MyPaginate {}

#[test]
fn test_trait_interface() {
    let my_paginate = MyPaginate { page: 1 };
    my_paginate.set_page(2);
    my_paginate.set_perpage(100);
    my_paginate.set_skip_page(20);
}

fn sum<T: Add<T, Output = T>>(a: T, b: T) -> T {
    a + b
}

fn sum2<T>(a:T, b:T) -> T where T: Add<T, Output = T> {
    a + b
}

#[test]
fn test_sum() {
    assert_eq!(sum(1, 2), 3);
    assert_eq!(sum(1u8, 2u8), 3);
}

#[derive(Debug)]
struct Foo;
trait Bar {
    fn baz(&self);
}

impl Bar for Foo {
    fn baz(&self) {
        println!("{:?}", self);
    }
}

fn static_dispatch<T>(t: &T) where T: Bar {
    t.baz();
}

fn dyn_dispatch(t: &dyn Bar) {
    t.baz();
}

#[test]
fn test_dispatch() {
    let foo = Foo;
    static_dispatch(&foo);
    dyn_dispatch(&foo);
}

trait Foo1: Sized {
    fn baz(&self);
}












































