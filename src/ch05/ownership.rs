#[test]
fn test_ownership() {
    let orig = Box::new(5);
    println!("{}", *orig);
    let stolen = orig;
    // println!("{}", *orig);  // ownership发生转移，不能够再使用
}

#[test]
fn test_ownership2() {
    let outer_val = 1;
    let outer_sp = "hello".to_string();
    {
        let inner_val = 2;
        outer_val;
        outer_sp;
    }
    println!("{}", outer_val);
    // println!("{:?}", outer_sp);   // 引用类型，在{}发生所有权转移
    // println!("{:?}", inner_val);  // 不在作用域的范围内
}

#[test]
fn test_match_ownership() {
    let a = Some("Rust".to_string());
    
    match a {
        Some(s) => println!("{:?}", s),
        _ => println!("nothing"),
    }
    
    // println!("a: {:?}", a);
}

#[test]
fn test_ownership_for() {
    let v = vec![1, 2, 3];
    for i in v {
        println!("i: {:?}", i);
        // println!("v: {:?}", v);
    }
}

#[test]
fn test_ownership_if_let() {
    let a = Some("hello".to_string());
    if let Some(s) = a {
        println!("s: {:?}", s);
    }
}

#[test]
fn test_ownership_while_let_option_i32() {
    let mut optional = Some(0);
    while let Some(i) = optional {
        if i > 9 {
            println!(">9,quit!");
            optional = None;
        } else {
            println!("i: {:?}, try again!", i);
            optional = Some(i + 1);
        }
    }
}

#[test]
fn test_ownership_closure() {
    let s = "hello ".to_string();
    let join = |i: &str| {s + i};
    assert_eq!("hello rust", join("rust"));
    // println!("s: {:?}", s);
}

fn modify_array(mut a : [i32; 3]) -> [i32;3] {
    a[0] = 3;
    a
}

#[test]
fn test_ownership_array() {
    let a = [1,2,3];
    let a1 = modify_array(a);
    assert_eq!([1,2,3], a);
    assert_eq!([3,2,3], a1);
}

fn update_array(v: &mut [i32;4]) {
    v[0] = 3;
}

#[test]
fn test_mut_array() {
    let mut v = [1,2,3,4];
    update_array(&mut v);
    assert_eq!([3,2,3,4], v);
}


fn bubble_sort(a: &mut Vec<i32>) {
    let mut n = a.len();
    
    while n > 0 {
        let (mut i, mut max_ptr) = (1, 0);
        while i < n {
            if a[i-1] > a[i] {
                a.swap(i-1, i);
                max_ptr = i;
            }
            i += 1;
        }
        n = max_ptr;
    }
}

#[test]
fn test_bubble_sort() {
    let mut a = vec![1,2,9,8,7,5];
    bubble_sort(&mut a);
    println!("a: {:?}", a);
}


fn compute(input: &i32, output: &mut i32) {
    if *input > 10 {
        *output = 1;
    }
    if *input > 5 {
        *output *= 2;
    }
}

fn compute2(input: &i32, output: &mut i32) {
    let cached = *input;
    if cached > 10 {
        *output = 2;
    } else if cached > 5 {
        *output *= 2;
    }
}

#[test]
fn test_compute() {
    let i = 20;
    let mut o = 5;
    compute(&i, &mut o);
    println!("i: {}", i);
    println!("o: {}", o);
}

#[test]
fn test_compute_2() {
    let mut i = 20;
    // 不能同时为可变借用和不可变借用；共享不可变，可变不共享
    // compute(&i, &mut i);
}

#[test]
fn test_compute2_1() {
    let i = 20;
    let mut o = 5;
    compute(&i, &mut o);
}

// deref会获得所有权，但是不可变引用不可以move，编译失败
fn join(s: &String) {
    // let append = *s;
}