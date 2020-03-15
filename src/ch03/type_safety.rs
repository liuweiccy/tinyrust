use std::fmt::Debug;

/// 标准的对象安全的trait
/// - trait的Self类型参数不能被限定为Sized
/// - trait中所有的方法都必须是对象安全的(任一条件)
///     - 方法受Self:Sized约束
///     - 方法签名同时满足
///         - 不能包含任何泛型参数
///         - 第一个参数必须为Self类型或可以解引用为Self的类型
///         - Self不能出现在出第一个参数之外的地方
///
/// - trait中不能够包含关联常量
trait Bar {
    fn bax(self, x: u32);
    fn bay(&self);
    fn baz(&mut self);
}

struct Name {
    name: String,
}

trait Foo1 {
    fn bad<T>(&self, x: T);
}

trait Foo {
    // 该方法在在trait对象不能够被调用
    fn new() -> Self
    where
        Self: Sized;
    fn say(&self);
}

impl Foo for Name {
    fn new() -> Self
    where
        Self: Sized,
    {
        Name {
            name: "rust".to_string(),
        }
    }

    fn say(&self) {
        println!("name: {:?}", self.name);
    }
}

impl Foo1 for Name {
    fn bad<T>(&self, x: T) {
        x;
    }
}

fn dyn_dispatch(d: &dyn Foo) {
    d.say();
}

#[test]
fn test_trait_safety() {
    let name: Name = Foo::new();
    dyn_dispatch(&name);
}
