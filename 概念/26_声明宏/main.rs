// 声明宏，使用macro_rules!进行定义，但是要保证宏定义必须在运行的函数之前
macro_rules! my_macro {
    () => {
        println!("声明宏执行");
    };
    //宏中可以写入多个函数，用于函数的重载匹配。
    (&val:expr) => {
        println!("声明宏带参数执行");
    };
}

fn main() {
    //调用宏时，要加!号，例如vec宏就是用于将数组变为可变数组，使用时要加!号
    my_macro!();
    test();
    my_macro3!();
}
macro_rules! my_macro2 {
    //宏定义必须在运行的函数之前
    () => {
        println!("宏2执行");
    };
}
fn test() {
    my_macro2!();
}
//在模块中声明的声明宏，要想在外部使用，需要通过过程宏#[macro_export]进行暴露
mod module1 {
    #[macro_export]
    macro_rules! my_macro3 {
        () => {
            println!("模块中的宏执行")
        };
    }
}
