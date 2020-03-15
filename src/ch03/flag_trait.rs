use std::thread;
use std::rc::Rc;

fn test_copy<T: Copy>(i: T) {
    println!("hhh");
}

#[test]
fn test_check_impl_copy() {
    let a = "String".to_string();
    test_copy("ss");
}

#[test]
fn test_thread() {
    let x = vec![1,2,3,4];
    thread::spawn(|| x);
}

#[test]
fn test_thread2() {
    let mut x = vec![1,2,3,4];
    thread::spawn(move || {
        x.push(2);
    });
    //x.push(5);
    //println!("{:?}", x);
}

#[test]
fn test_no_send_sync() {
    let x = Rc::new(vec![1,2,3,4]);
    thread::spawn(move || {
        x[1];
    });
}

