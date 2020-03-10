fn move_coords(x: (i32, i32)) -> (i32, i32) {
    (x.0 + 1, x.1 + 1)
}

#[test]
fn test_tuple() {
    let tuple: (&'static str, i32, char) = ("hello", 5, 'c');
    assert_eq!(tuple.0, "hello");
    assert_eq!(tuple.1, 5);
    assert_eq!(tuple.2, 'c');

    let coords = (0, 1);
    let result = move_coords(coords);
    assert_eq!(result, (1, 2));

    let (x, y) = move_coords(coords);
    assert_eq!(x, 1);
    assert_eq!(y, 2);
}

#[derive(Debug, PartialEq)]
struct People {
    name: &'static str,
    gender: u32,
}

impl People {
    fn new(name: &'static str, gender: u32) -> Self {
        return People { name, gender };
    }

    fn name(&self) {
        println!("name: {:?}", self.name);
    }

    fn set_name(&mut self, name: &'static str) {
        self.name = name;
    }

    fn gender(&self) {
        let gender = if self.gender == 1 { "boy" } else { "girl" };

        println!("gender: {:?}", gender);
    }
}

#[test]
fn test_struct() {
    let mut p = People::new("liu", 1);
    p.name();
    p.set_name("vv");
    p.name();
    p.gender();
}

// 元组结构体
struct Color(i32, i32, i32);

#[test]
fn test_tuple_struct() {
    let color = Color(0, 1, 2);
    assert_eq!(color.0, 0);
    assert_eq!(color.1, 1);
    assert_eq!(color.2, 2);
}

// new type:是一种新的类型，表现的方式和
struct Integer(u32);
// alias
type Int = i32;
#[test]
fn test_new_type() {
    let int = Integer(10);
    assert_eq!(int.0, 10);
    let int: Int = 10;
    assert_eq!(int, 10);
}

// 单元结构体
struct Empty;

#[test]
fn test_empty_struct() {
    let x = Empty;
    println!("{:p}", &x);

    let y = x;
    println!("{:p}", &y);

    let z = Empty;
    println!("{:p}", &z);

    assert_eq!((..), std::ops::RangeFull);
}


enum Number {
    Zero,
    One,
    Two,
}

#[test]
fn test_enum() {
    let a = Number::One;
    match a {
        Number::Zero => println!("0"),
        Number::One => println!("1"),
        Number::Two => println!("2"),
    }
}
