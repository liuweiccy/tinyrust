#[test]
fn test_bool() {
    let x = true;
    let y: bool = false;
    let x = 5;
    if x > 1 {
        println!("x is bigger than 1");
    }

    let x = true;
    assert_eq!(x as i32, 1);
    assert_eq!(y as i32, 0);
}

#[test]
fn test_number() {
    let num = 42u32;
    let num: u32 = 42;
    let num = 0x2a;
    let num = 0o71;
    let num = 0b1100_1010;

    println!("{}", b'*');
    assert_eq!(b'*', 42u8);
    assert_eq!(b'\'', 39u8);
    assert_eq!(b',', 44u8);

    let num = 3.1415926f64;
    assert_eq!(-3.14, -3.14f64);
    assert_eq!(2., 2.0f64);
    assert_eq!(2e4, 20000f64);

    println!("{:?}", std::f32::INFINITY);
    println!("{:?}", std::f32::NEG_INFINITY);
    println!("{:?}", std::f32::NAN);
    println!("{:?}", std::f32::MAX);
    println!("{:?}", std::f32::MIN);
}

#[test]
fn test_char() {
    let x = 'r';
    let x = 'の';

    println!("{}", "\'");
    println!("{}", "\\");
    println!("{}", "\n");
    println!("{}", "\r");
    println!("{}", "\t");

    assert_eq!('\x2a', '*');
    assert_eq!('\x25', '%');

    let c = '\u{CA0}';
    println!("{}", c);

    let c = '\u{151}';
    println!("{}", c);

    assert_eq!('%' as i8, 37);

    let c = '刘';
    println!("{:x}", c as i64)
}

#[test]
fn test_array() {
    let arr: [i32; 3] = [1, 2, 3];
    let mut mut_arr = [1, 2, 3];

    assert_eq!(1, mut_arr[0]);

    mut_arr[0] = 3;
    assert_eq!(mut_arr[0], 3);

    let init_arr = [0; 10];
    println!("{:?}", init_arr);
    assert_eq!(init_arr[5], 0);
    assert_eq!(init_arr.len(), 10);

    // 越界访问数组，编译报错
    // println!("{:?}", arr[5]);
}

#[test]
fn test_range() {
    assert_eq!((1..5), std::ops::Range { start: 1, end: 5 });
    assert_eq!((1..=5), std::ops::RangeInclusive::new(1, 5));
    assert_eq!(3 + 4 + 5, (3..6).sum());
    assert_eq!(3 + 4 + 5 + 6, (3..=6).sum());
    assert_eq!(1 * 2 * 3 * 4, (1..5).product());

    for i in 1..5 {
        println!("{}", i);
    }

    for i in 1..=5 {
        println!("{}", i)
    }
}

#[test]
fn test_slice1() {
    let arr: [i32; 5] = [1, 2, 3, 4, 5];
    assert_eq!(&arr, &[1, 2, 3, 4, 5]);
    assert_eq!(&arr[1..], [2, 3, 4, 5]);
    assert_eq!(&arr.len(), &5);
    assert_eq!(&arr.is_empty(), &false);

    let arr = &mut [1, 2, 3];
    arr[1] = 7;
    assert_eq!(arr, &[1, 7, 3]);

    let vec = vec![1, 2, 3];
    assert_eq!(&vec[..], [1, 2, 3]);
}

// unstable功能,暂时不能够使用
// #[feature(core_intrinsics)]
// fn print_type_of<T>(_ : &T) {
//     println!("{}", unsafe{ std::intrinsics::type_name::<T>()})
// }

#[test]
fn test_str() {
    let truth: &'static str = "Rust是一门优雅的语言";
    let ptr = truth.as_ptr();
    let len = truth.len();
    assert_eq!(28, len);
    let s = unsafe {
        let slice = std::slice::from_raw_parts(ptr, len);
        std::str::from_utf8(slice)
    };

    assert_eq!(s, Ok(truth));
}

#[test]
fn test_raw_pointer() {
    let mut x = 10;
    let ptr_x = &mut x as *mut i32;
    println!("{:p}", ptr_x);

    let y = Box::new(20);
    let ptr_y = &*y as *const i32;
    unsafe {
        *ptr_x += *ptr_y;
    }
    assert_eq!(x, 30);
}

// #[feature(never_type)]
// fn foo() -> u32 {
//     let x : ! = {
//         return 123
//     };
// }

#[test]
fn test_never() {
    let num: Option<u32> = Some(42);
    match num {
        Some(num) => num,
        None => panic!("Nothing"),
    };
}
