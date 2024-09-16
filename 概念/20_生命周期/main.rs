/**
 * 生命周期是用于描述引用有效性范围的概念，用于确保引用不会变成悬空引用（即引用无效或已释放内存的指针）
 */
//这样编译器可以保证longest函数返回的引用在函数调用期间是有效的// fn longest(x: &str, y: &str) -> &str { rust无法保证参数的引用在函数调用结束后依然有效
//     if x.len() > y.len() {
//         x
//     } else {
//         y
//     }
// }
// 生命周期'a表示：返回的引用&'a str只会在x和y的生命周期内有效
//这样编译器可以保证longest函数返回的引用在函数调用期间是有效的
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}
fn main() {
    println!("{}--{}", longest("abcd", "123"), "abcd");
    println!("{}--{}", longest("abc", "1234"), "1234");
    //静态生命周期,表示程序运行整个时间都存在
    static HELLO: &str = "你好世界";
    println!("{HELLO}");
}
