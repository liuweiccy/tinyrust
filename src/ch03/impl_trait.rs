use std::fmt::Debug;

pub trait Fly {
    fn fly(&self) -> bool;
}

#[derive(Debug)]
struct Duck;
#[derive(Debug)]
struct Pig;

impl Fly for Duck {
    fn fly(&self) -> bool {
        return true;
    }
}

impl Fly for Pig {
    fn fly(&self) -> bool {
        false
    }
}


fn fly_static<T: Fly>(x : T) -> bool {
    x.fly()
}

// impl trait
fn fly_static2(x: impl Fly + Debug) -> bool {
    x.fly()
}

// impl trait
fn can_fly(s: impl Fly + Debug) -> impl Fly {
    if s.fly() {
        println!("{:?} can fly", s);
    } else {
        println!("{:?} can't fly", s);
    }
    s
}

#[test]
fn test_impl_trait() {
    let pig = Pig;
    assert_eq!(fly_static(pig), false);
    let pig = Pig;
    assert_eq!(fly_static2(pig), false);
    let duck = Duck;
    assert_eq!(fly_static(duck), true);
    
    let pig = Pig;
    let pig = can_fly(pig);
    let duck = Duck;
    let duck = can_fly(duck);
    let s : impl Fly + Debug = Pig;
    println!("{:?}", s);
}