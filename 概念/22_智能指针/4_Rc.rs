// Rc智能指针用于提供对某个值的共享所有权，它允许多个所有者共同拥有对同一数据的不可变访问权
use std::rc::Rc;
fn main() {
    let data = Rc::new(10); // 创建一个新的 Rc<T>

    {
        let data1 = Rc::clone(&data); // 增加引用计数
        println!("data1: {}", *data1);
    } // data1 离开作用域，但 data 仍然存在

    println!("data: {}", *data); // 继续使用 data
} // data 离开作用域，引用计数归零，数据被清理
