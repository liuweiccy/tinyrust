use std::rc::Rc;
use std::ops::{Deref, Add};
use std::borrow::Borrow;

#[test]
fn test_string_deref() {
    let a = "hello".to_string();
    let b = " world".to_string();
    let c = a + &b;
    println!("{:?}", c);
}


fn foo(s: &[i32]) {
    println!("{:?}", s[0])
}

#[test]
fn test_vec_deref() {
    let v = vec![1,2,3,4];
    foo(&v);
}


#[test]
fn test_rc_deref() {
    let x = Rc::new("hello");
    println!("{:?}", x.chars());
}


#[test]
fn test_hm_clone_deref() {
    let x = Rc::new("hello");
    
    // 因为同时实现了clone方法，所以不能够自动的解引用
    let y = x.clone();
    let z = (*x).clone();
}

#[test]
fn test_match_deref() {
    let x = "hello".to_string();
    match x.deref() {
        "hello" => println!("hello"),
        _ => {},
    }
    
    match x.as_ref() {
        "hello" => println!("hello"),
        _ => {},
    }
    
    match x.as_str() {
        "hello" => println!("hello"),
        _ => {},
    }
    
    match x.borrow() {
        "hello" => println!("hello"),
        _ => {},
    }
    
    match &(*x) {
        "hello" => println!("hello"),
        _ => {},
    }
    
    match &x[..] {
        "hello" => println!("hello"),
        _ => {},
    }
}

#[test]
fn test_as1() {
    let a = 1u32;
    let b = a as u64;
    let c = 3u64;
    let d = c as u8;
}

#[test]
fn test_as2() {
    let a: u32 = std::u32::MAX;
    let b = a as u16;
    println!("{}", a);
    println!("{}", b);
    
    let e = -1i32;
    let f = e as u32;
    println!("{:?}", e);
    println!("{:?}", f);
}

struct Stu(i32);

trait A {
    fn test(&self, i: i32);
}
trait B {
    fn test(&self, i: i32);
}

impl A for Stu {
    fn test(&self, i: i32) {
        println!("From A: {:?}", i);
    }
}

impl B for Stu {
    fn test(&self, i: i32) {
        println!("From B: {:?}", i + 1);
    }
}

#[test]
fn test_trait_same_method() {
    let s = Stu(1);
    A::test(&s, 1);
    B::test(&s, 1);
    <Stu as A>::test(&s, 1);
    <Stu as B>::test(&s, 1);
}

#[test]
fn test_lifetime() {
    let a : &'static str = "hello";
    let b : &str = a as &str;
    let c : &'static str = b as &'static str;
}

#[test]
fn test_from() {
    let string = "hello".to_string();
    let other_string = String::from("hello");
    assert_eq!(string, other_string);
}

#[derive(Debug)]
struct Person{
    name: String,
}

impl Person {
    fn new<T: Into<String>>(name: T) -> Person {
        Person { name: name.into() }
    }
}

#[test]
fn test_into() {
    let person = Person::new("Alex");
    let person = Person::new("Alex".to_string());
    println!("{:?}", person);
}

#[derive(PartialEq)]
struct Int(i32);
impl Add<i32> for Int {
    type Output = i32;
    
    fn add(self, rhs: i32) -> Self::Output {
        self.0 + rhs
    }
}


// 违反孤儿原则
// impl Add<i32> for Option<Int> {
//     type Output = i32;
//
//     fn add(self, rhs: i32) -> Self::Output {
//         self.unwrap().0 + rhs
//     }
// }


// 因为Box有标志属性#[fundamental]所以不受孤儿原则影响
impl Add<i32> for Box<Int> {
    type Output = i32;
    
    fn add(self, rhs: i32) -> Self::Output {
        self.0 + rhs
    }
}

#[test]
fn test_local_type() {
    assert_eq!(Int(3) + 3, 6);
    assert_eq!(Box::new(Int(3)) + 3, 6);
}

#[feature(specialization)]
struct Diver<T> {
    inner: T,
}

trait Swimmer {
    fn swim(&self) {
        println!("swimming");
    }
}

impl <T> Swimmer for Diver<T> { }

// stable版本不支持impl特化
// impl Swimmer for Diver<&'static str> {
//     fn swim(&self) {
//         println!("drowning, help!");
//     }
// }

#[test]
fn test_impl_specialization() {
    let x = Diver::<&'static str> { inner: "Bob" };
    x.swim();
    
    let y = Diver::<String> { inner: "Alice".to_string() };
    y.swim();
}