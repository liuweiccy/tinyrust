fn foo(x: u32) {
    let y = x;
    let z = 100;
}

#[test]
fn test_stack_frame() {
    let x = 42;
    foo(x);
}

struct A {
    a: u8,
    b: u32,
    c: u16,
}

#[test]
fn test_size_of() {
    println!("{:?}", std::mem::size_of::<A>());
}

union U {
    f1: u32,
    f2: f32,
    f3: f64,
}

#[test]
fn test_size_of_union() {
    println!("{:?}", std::mem::size_of::<U>());
}

// 全局，内联编译
const i: i32 = 18 + 20;
// 全局，分配到栈
static j: u32 = 20;

#[test]
fn test_local_malloc() {
    let x: i32 = 1;
    println!("{}", x);
}

#[test]
fn test_if_malloc() {
    let x: i32;
    if true {
        x = 10;
    } else {
        x = 20;
    }
    println!("{:?}", x);
}

#[test]
fn test_vec_init_type() {
    let a: Vec<i32> = vec![];
    let b: [i32; 0] = [];
}

#[test]
fn test_move_init() {
    let x = 42;
    let y = Box::new(5);
    println!("{:p}", y);
    let x2 = x;
    let y2 = y;
    // move
    // println!("{:?}", y);
}

#[test]
fn test_smart_point_string_vec() {
    let s = String::from("hello");
    // 解引用后不能够确定大小所以编译器报错
    // let deref_s = *s;

    let v = vec![1, 2, 3];
    // let deref_vec = *v;
}

#[derive(Debug)]
struct S(i32);

impl Drop for S {
    fn drop(&mut self) {
        println!("drop: {:?}", self.0);
    }
}

#[test]
fn test_drop() {
    let x = S(1);
    println!("crate x: {:?}", x);
    {
        let y = S(2);
        println!("crate y: {:?}", y);
        println!("exit inner scope");
    }
    println!("exit main");
}

#[test]
fn test_active() {
    let mut v = vec![1, 2, 3];
    {
        v
    };
    // 在前面的作用域
    // v.push(2);
}

#[test]
fn test_shadowing_drop() {
    let x = S(1);
    let x = S(2);
}

// 标记了变量的生命周期，局部变量在函数结束后将被销毁
// fn foo2<'a>() -> &'a str {
//     let a = "hello".to_string();
//     &a
// }

struct A1 {
    a: u32,
    b: Box<u64>,
}
struct B1(i32, f64, char);
struct N;
enum E {
    H(u32),
    M(Box<u32>),
    S(i8),
}
union U1 {
    u: u32,
    v: u64,
}

#[test]
fn test_malloc_stack_or_heap() {
    println!("Box<u32>: {:?}", std::mem::size_of::<Box<u32>>()); // 8
    println!("A1: {:?}", std::mem::size_of::<A1>()); // 16
    println!("B1: {:?}", std::mem::size_of::<B1>()); // 16
    println!("N: {:?}", std::mem::size_of::<N>()); // 0
    println!("E: {:?}", std::mem::size_of::<E>()); // 16
    println!("U1: {:?}", std::mem::size_of::<U1>()); // 8
}
