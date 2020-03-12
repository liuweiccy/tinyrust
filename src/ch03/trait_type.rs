trait Add<RHS, Output> {
    fn add(self, rhs: RHS) -> Output;
}

impl Add<i32, i32> for i32 {
    fn add(self, rhs: i32) -> i32 {
        self + rhs
    }
}

impl Add<u32, i32> for u32 {
    fn add(self, rhs: u32) -> i32 {
        (self + rhs) as i32
    }
}

#[test]
fn test_add_trait() {
    let (a, b, c, d) = (1i32, 2i32, 3u32, 4u32);
    let x = a.add(b);
    let y = c.add(d);

    assert_eq!(x, 3);
    assert_eq!(y, 7);
}

#[test]
fn test_add_trait2() {
    let a = "hello ";
    let b = "rust";
    println!("{}", a.to_string() + b);
}
