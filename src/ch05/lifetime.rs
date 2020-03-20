#[test]
fn test_borrow_check() {
    let r;
    {
        let x = 5;
        r = &x;
    }
    // 'r大于'x，会造成悬垂指针
    // println!("r: {:?}", r);
}

// 不能对本地变量借用
// fn return_str<'a>() -> &'a str {
//     let mut s = "rust".to_string();
//     for i in 0..3 {
//         s.push_str("Good ");
//     }
//     &s[..]
// }
//
// #[test]
// fn test_lifetime() {
//     let s = return_str();
// }

fn the_longest<'a, 'b: 'a>(s1: &'a str, s2: &'b str) -> &'a str {
    if s1.len() > s2.len() {
        s1
    } else {
        s2
    }
}

#[test]
fn test_fn_longest() {
    let s1 = String::from("Rust");
    let s1_r = &s1;
    {
        let s2 = String::from("C");
        let s2_r = &s2;
        println!("{:?}", the_longest(s1_r, s2_r));
    }
}