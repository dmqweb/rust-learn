// struct Book<'a> {
//     //此结构体包含了对&str类型的两个字段的引用，但未指定生命周期
//     author: &str,
//     title: &str,
// }
struct Book<'a> {
    //使用&'a声明周期的引用，使得两字段具有实例相同生命周期范围的引用
    author: &'a str,
    title: &'a str,
}
fn main() {
    // 创建一个 Book 实例
    // 将生命周期 `'a` 应用于 Book 实例的字段
    let book = Book {
        author: "George Orwell",
        title: "1984",
    };
    println!("{} by {}", book.title, book.author);
}
