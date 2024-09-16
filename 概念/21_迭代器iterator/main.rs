fn main() {
    let fruits = ["苹果", "香蕉", "梨"];
    let mut iterator = fruits.iter();
    println!("{:?}", iterator);
    //迭代器生成的是数组对象的引用而非数值，需要使用&借用字符的引用
    println!("{}", iterator.next() == Some(&"苹果"));
    println!("{}", iterator.next() == Some(&"香蕉"));
    println!("{}", iterator.next() == Some(&"梨"));
    println!("{:?}", iterator.next());
}
