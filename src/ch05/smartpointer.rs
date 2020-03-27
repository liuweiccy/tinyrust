use futures::executor::block_on;
use std::thread::sleep;
use std::time::Duration;

#[test]
fn test_smart_pointer() {
    let x = Box::new("hello");
    let y = x;
    // println!("{:?}", x);
}

#[test]
fn test_smart_pointer_deref() {
    let a = Box::new("hello");
    let b = Box::new("Rust".to_string());
    let c = *a;
    let d = *b;
    println!("{:?}", a);
    // println!("{:?}", b);
}

async fn hello_world() {
    println!("hello world");
}


#[test]
fn test_async_hello_world() {
    let future = hello_world();
    block_on(future);
}

#[derive(Debug)]
struct Song(String);
async fn learn_song() -> Song {
    println!("learn song");
    Song("Ding".to_string())
}

async fn sing_song(song: Song) {
    println!("{:?}", song);
}
async fn dance() {
    println!("dance...");
}

#[test]
fn test_block() {
    let song = block_on(learn_song());
    block_on(sing_song(song));
    block_on(dance());
}

async fn learn_and_sing() {
    let song = learn_song().await;
    sing_song(song).await;
}

async fn learn_sing_dance() {
    let f1 = learn_and_sing();
    let f2 = dance();
    futures::join!(f1, f2);
}

#[test]
fn test_async() {
    block_on(learn_sing_dance());
}