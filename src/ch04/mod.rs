fn foo(x: u32) {
    let y = x;
    let z = 100;
}

#[test]
fn test_stack_frame() {
    let x = 42;
    foo(x);
}