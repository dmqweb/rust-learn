fn main() {
    //定义类型变量
    let ten: i32 = 10;
    //类型自动推断
    let one = 2;
    //定义常量,全部大写,必须要指定类型
    const MAX_NUM: i32 = 100_000;
    println!("{}{}{}", ten, one, MAX_NUM);
}
