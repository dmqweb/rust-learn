//使用trait关键字定义特质
trait AppendBar {
    fn append_bar(self) -> Self;
}
//使用impl为某个类型实现trait
impl AppendBar for String {
    fn append_bar(self) -> Self {
        // 创建一个新的字符串，其中包含原始字符串和 "Bar"
        let mut new_string = self;
        new_string.push_str("Bar");
        new_string
    }
}
fn main() {
    let s = String::from("Foo");
    let s = s.append_bar();
    println!("s: {}", s); // 使用 {} 插值方式打印
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn is_foo_bar() {
        assert_eq!(String::from("Foo").append_bar(), "FooBar");
    }
    #[test]
    fn is_bar_bar() {
        assert_eq!(String::from("").append_bar().append_bar(), "BarBar");
    }
}
