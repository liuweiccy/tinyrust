#[test]
fn test_scope() {
    let v = "hello, world";
    assert_eq!(v, "hello, world");

    let v = "hello, Rust!";
    assert_eq!(v, "hello, Rust!");

    {
        let v = "hello world";
        assert_eq!(v, "hello world");
    }

    assert_eq!(v, "hello, Rust!");
}

pub fn math(op: fn(i32, i32) -> i32, a: i32, b: i32) -> i32 {
    op(a, b)
}

fn sum(a: i32, b: i32) -> i32 {
    a + b
}

fn product(a: i32, b: i32) -> i32 {
    a * b
}

#[test]
fn test_func_args() {
    let a = 2;
    let b = 3;

    assert_eq!(math(sum, a, b), 5);
    assert_eq!(math(product, a, b), 6);
}

fn is_true() -> bool {
    true
}

fn true_maker() -> fn() -> bool {
    is_true
}

#[test]
fn test_func_return() {
    assert_eq!(true_maker()(), true);
}

// 编译时函数执行(CTFE)
const fn init_len() -> usize {
    5
}

#[test]
fn test_ctfe_const() {
    let arr = [0; init_len()];
}

#[test]
fn test_closure() {
    let out = 42;
    fn add(i: i32, j: i32) -> i32 {
        i + j
    }

    // 不是闭包不能够在函数内使用外部变量
    // fn add1(i:i32, j:i32) -> i32 {
    //     i + j + out
    // }

    let closure_annotated = |i: i32, j: i32| -> i32 { i + j + out };
    let closure_inferred = |i, j| i + j + out;

    let i = 1;
    let j = 2;

    assert_eq!(add(i, j), 3);
    assert_eq!(closure_annotated(i, j), 45);
    assert_eq!(closure_inferred(i, j), 45);
}

// 闭包的声明与定义
fn closure_math<F: Fn() -> i32>(op: F) -> i32 {
    op()
}

#[test]
fn test_closure_args() {
    let a = 2;
    let b = 3;
    assert_eq!(closure_math(|| a + b), 5);
    assert_eq!(closure_math(|| a * b), 6);
}

fn two_times_impl() -> impl Fn(i32) -> i32 {
    let i = 2;
    move |j| i * j
}

#[test]
fn test_closure_return() {
    let result = two_times_impl();
    assert_eq!(result(2), 4);
}
