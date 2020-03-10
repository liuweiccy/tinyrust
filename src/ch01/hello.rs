pub fn say(name: &str) {
    println!("hello, {}", name);
}

#[test]
fn test_say() {
    say("rust");
}
