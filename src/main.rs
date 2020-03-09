use std::collections::HashMap;
use std::cmp::min;

mod ch01;

fn main() {
    println!("Hello, world!");
    ch01::hello::say("rust!");

    let mut map = HashMap::new();
    map.insert(1, 2);

    for i in 1 .. 10000 {
        map.insert(i, 20);
    }

    println!("map len:{}", map.len());

    for v in map.iter() {
        println!("{}--{}", v.0, v.1);
    }

    let mut x = 0;
    loop {
        if x == 10 {
            break;
        } else {
            x += 1;
        }
    }

    let x = vec![1,2,3,4];
    let y = x;
    //println!("{:?}", x);
    println!("{:?}", y);

    let data1 = &[3,1,4,1,5,9,2,6];
    let data2 = &[2,7,1,8,2,8,1,8];
    let numbers = data1.iter()
        .zip(data2.iter())
        .map(|(a, b)| a * b)
        .filter(|n| *n > 5)
        .take(4)
        .collect::<Vec<_>>();
    println!("{:?}", numbers);


    let mut numbers = Vec::new();
    for i in 0 .. min(data1.len(), data2.len()) {
        let n = data1[i] * data2[i];
        if n > 5 {
            numbers.push(n);
        }
        if numbers.len() == 4 {
            break;
        }
    }
    println!("{:?}", numbers);
}