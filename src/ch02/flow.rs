#[test]
fn test_if() {
    let n = 13;
    let big_n = if n < 10 && n > -10 { 10 * n } else { n / 2 };

    assert_eq!(big_n, 6);
}

#[test]
fn test_for() {
    for n in 1..101 {
        if n % 15 == 0 {
            println!("fizzbuzz");
        } else if n % 3 == 0 {
            println!("fizz");
        } else if n % 5 == 0 {
            println!("buzz");
        } else {
            println!("{}", n);
        }
    }
}

fn while_true(x: i32) -> i32 {
    loop {
        return x + 1;
    }
    x
}

#[test]
fn test_while() {
    let y = while_true(5);
    assert_eq!(y, 6);
}

#[test]
fn test_match() {
    let number = 42;
    match number {
        0 => println!("Origin"),
        1...3 => println!("All"),
        5 | 7 | 13 => println!("Bad Luck"),
        n @ 42 => println!("Answer is {}", n),
        _ => println!("Common"),
    }
}

#[test]
fn test_if_let() {
    let boolean = true;
    let mut binary = 0;

    if let true = boolean {
        binary = 1;
    }

    assert_eq!(binary, 1);
}

#[test]
fn test_while_no_let() {
    let mut v = vec![1, 2, 3, 4, 5, 6];

    loop {
        match v.pop() {
            Some(x) => println!("{}", x),
            None => break,
        }
    }
}

#[test]
fn test_while_let() {
    let mut v = vec![1, 2, 3, 4, 5, 6, 7, 8];
    while let Some(x) = v.pop() {
        println!("{}", x);
    }

    println!("len: {}", v.len());
    println!("cap: {}", v.capacity());
}
