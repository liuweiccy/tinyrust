#[test]
fn test_result() {
    let x: Result<i32, &str> = Ok(-3);
    assert_eq!(x.is_ok(), true);

    let x: Result<i32, &str> = Err("不能够为负数");
    assert_eq!(x.is_ok(), false);
}
