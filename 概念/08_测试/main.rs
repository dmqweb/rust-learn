fn main() {}
//测试函数
fn bigger(a: i32, b: i32) -> i32 {
    if a - b > 0 {
        a
    } else {
        b
    }
}
//属性宏,当执行test命令,进行单元检测时才运行下面代码
#[cfg(test)]
//模块定义,创建了一个tests子模块
mod tests {
    // 当该模块的父模块进行引入，也可以只引入某个变量或函数
    use super::*;
    //标记其后的函数是一个测试函数
    #[test]
    fn ten_is_bigger_than_eight() {
        //断言宏
        assert_eq!(10, bigger(10, 8));
    }
    #[test]
    fn fortytwo_is_bigger_than_thirtytwo() {
        assert_eq!(42, bigger(32, 42));
    }

    #[test]
    fn equal_numbers() {
        assert_eq!(42, bigger(42, 42));
        panic!("我光报错");
    }
}
