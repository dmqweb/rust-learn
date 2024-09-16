#[derive(Debug)]
// 泛型使用
struct Wrapper<T> {
    value: T,
}
//需要使用impl实现结构体，这样才能使用new关键字创建对应的实例
impl<T> Wrapper<T> {
    fn new(value: T) -> Self {
        Wrapper { value }
    }
}
fn main() {
    let mut numbers: Vec<i32> = Vec::new();
    let n1: u8 = 42;
    numbers.push(n1.into());
    let n2: i8 = -1;
    numbers.push(n2.into());
    println!("{numbers:?}");
    println!("{:?}", Wrapper::new(34));
    println!("{}", Wrapper::new(42).value == 42);
    println!("{}", Wrapper::new("Foo").value == "Foo");
}
