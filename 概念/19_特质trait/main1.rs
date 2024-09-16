//使用trait关键字定义特质
trait AppendBar {
    fn append_bar(self) -> Self;
}
//使用impl关键字给某类型实现某特质
impl AppendBar for Vec<String> {
    fn append_bar(mut self) -> Self {
        //self关键字和Self关键字用于表示当前对象实例和当前对象类型
        self.push(String::from("Bar"));
        self
    }
}
fn main() {
    println!("{:?}", vec![String::from("Foo")].append_bar());
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn is_vec_pop_eq_bar() {
        let mut foo = vec![String::from("Foo")].append_bar();
        assert_eq!(foo.pop().unwrap(), "Bar");
        assert_eq!(foo.pop().unwrap(), "Foo");
    }
}
