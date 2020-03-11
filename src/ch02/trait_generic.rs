use std::fmt::{Debug, Display, Formatter, Error};

fn match_option<T: Debug + Display>(o: Option<T>) {
    match o {
        Some(i) => println!("{:?}", i),
        None => println!("nothing"),
    }
}

#[test]
fn test_option_generic() {
    let a: Option<i32> = Some(3);
    let b: Option<&str> = Some("hello");
    let c: Option<char> = Some('c');
    let d: Option<u32> = None;

    match_option(a);
    match_option(b);
    match_option(c);
    match_option(d);
}


#[derive(Clone)]
struct Duck{
    name: String,
}
#[derive(Clone)]
struct Pig {
    name : String,
}

trait Fly {
    fn fly(&self) -> bool;
}

impl Fly for Duck {
    fn fly(&self) -> bool {
        true
    }
}

impl Fly for Pig {
    fn fly(&self) -> bool {
        false
    }
}

fn fly_static<T : Fly>(s : T) -> bool {
    s.fly()
}

fn fly_dyn(s : &dyn Fly) -> bool {
    s.fly()
}

#[test]
fn test_trait() {
    let pig = Pig{name:"Petch".to_string()};
    assert_eq!(fly_static(pig.clone()), false);
    
    let duck = Duck{name:"GaGa".to_string()};
    assert_eq!(fly_static(duck.clone()), true);
    assert_eq!(fly_dyn(&duck), true);
    assert_eq!(fly_dyn(&pig), false);
}

struct Point {
    x:i32,
    y:i32,
}

impl Debug for Point {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        write!(f, "Point {{x:{}, y:{}}}", self.x, self.y)
    }
}

#[test]
fn test_debug_trait() {
    let origin = Point{ x: 0, y: 0 };
    println!("{:?}", origin);
}
