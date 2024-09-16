// Box智能指针用于在堆上分配内存，从而：1 避免编译时无法确定类型大小、2 自动管理内存分配和释放，避免内存泄漏、3 减小栈的压力，提高程序性能
#[derive(PartialEq, Debug)]
enum List {
    Cons(i32, Box<List>),
    Nil,
}
// 创建一个空的 cons 列表
fn create_empty_list() -> List {
    List::Nil
}
// 创建一个非空的 cons 列表
fn create_non_empty_list() -> List {
    List::Cons(1, Box::new(List::Nil))
}
fn main() {
    println!("This is an empty cons list: {:?}", create_empty_list());
    println!(
        "This is a non-empty cons list: {:?}",
        create_non_empty_list(),
    );
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_create_empty_list() {
        assert_eq!(create_empty_list(), List::Nil);
    }
    #[test]
    fn test_create_non_empty_list() {
        assert_ne!(create_empty_list(), create_non_empty_list());
    }
}
