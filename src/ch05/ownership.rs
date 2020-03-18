#[test]
fn test_ownership() {
    let orig = Box::new(5);
    println!("{}", *orig);
    let stolen = orig;
    // println!("{}", *orig);  // ownership发生转移，不能够再使用
}
