use std::thread::sleep;
use std::time::Duration;

#[test]
fn test_str() {
    let str = "Hello Rust";
    let ptr = str.as_ptr();
    let len = str.len();

    println!("{:p}", ptr);
    println!("{}", len);
}

fn reset1(mut arr: [u32; 5]) {
    arr[0] = 5;
    arr[1] = 4;
    arr[2] = 3;
    arr[3] = 2;
    arr[4] = 1;

    println!("reset arr {:?}", arr);
}

#[test]
fn test_array1() {
    let arr: [u32; 5] = [1, 2, 3, 4, 5];
    reset1(arr);
    println!("origin arr {:?}", arr);
}

fn reset2(arr: &mut [u32]) {
    arr[0] = 5;
    arr[1] = 4;
    arr[2] = 3;
    arr[3] = 2;
    arr[4] = 1;

    println!("reset arr {:?}", arr);
    println!("arr length {:?}", arr.len());
}

#[test]
fn test_array2() {
    let mut arr = [1, 2, 3, 4, 5];
    println!("reset before: origin arr {:?}", arr);
    reset2(&mut arr);
    println!("reset after: origin arr {:?}", arr);
}

#[test]
fn test_size() {
    assert_eq!(std::mem::size_of::<&[u32; 4]>(), 8);
    assert_eq!(std::mem::size_of::<&mut [u32]>(), 16);
}

/// 零大小类型，并不占用内存空间
enum Void {}
struct Foo;
struct Baz {
    foo: Foo,
    qux: (),
    baz: [u8; 0],
}

#[test]
fn test_zero_sized_type() {
    assert_eq!(std::mem::size_of::<()>(), 0);
    assert_eq!(std::mem::size_of::<Foo>(), 0);
    assert_eq!(std::mem::size_of::<Void>(), 0);
    assert_eq!(std::mem::size_of::<Baz>(), 0);
    assert_eq!(std::mem::size_of::<[u8; 0]>(), 0);
    assert_eq!(std::mem::size_of::<[(); 10]>(), 0);
}

#[test]
fn test_data_type() {
    let v: Vec<()> = vec![(); 10];

    for i in v {
        println!("{:?}", i);
    }
}

fn foo() -> ! {
    loop {
        println!("hello");
        sleep(Duration::new(1, 0));
    }
}

#[test]
fn test_never_type() {
    let i = if false { foo() } else { 100 };

    assert_eq!(i, 100);
}

#[test]
fn test_enum_void() {
    let res: Result<u32, Void> = Ok(0);
    if let Ok(num) = res {
        println!("{}", num);
    }
}

#[test]
fn test_type_infer() {
    let x = "1";
    let int_x: u8 = x.parse().unwrap();
    println!("{:?}", int_x);
    println!("{:?}", x.parse::<u16>().unwrap());
}

#[test]
fn test_type_infer2() {
    let a: f64 = 0.0;
    let a_pox = a.is_sign_positive();
}
