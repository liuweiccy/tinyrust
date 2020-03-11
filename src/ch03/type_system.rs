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
