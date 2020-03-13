trait Add<RHS = Self> {
    type Output;

    fn add(self, rsh: RHS) -> Self::Output;
}

impl Add<u64> for u32 {
    type Output = u64;

    fn add(self, rhs: u64) -> Self::Output {
        (self as u64) + rhs
    }
}

#[test]
fn test_ops_overload() {
    let a = 1u32;
    let b = 10u64;
    assert_eq!(a.add(b), 11);
}

#[derive(Debug)]
struct Point {
    x: i32,
    y: i32,
}

impl Add for Point {
    type Output = Point;

    fn add(self, rsh: Self) -> Self::Output {
        Point {
            x: self.x + rsh.x,
            y: self.y + rsh.y,
        }
    }
}

#[test]
fn test_trait_add() {
    println!("{:?}", (Point { x: 1, y: 2 }).add(Point { x: 2, y: 1 }));
}
