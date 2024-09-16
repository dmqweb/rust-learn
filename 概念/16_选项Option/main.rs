// Option选项类型，
fn maybe_icecream(hour_of_day: u16) -> Option<u16> {
    if hour_of_day > 23 {
        None
    } else if hour_of_day < 22 {
        Some(5)
    } else {
        Some(0)
    }
}
fn main() {
    //定义普通Option类型,通过Option声明选项类型通过Some函数创建选项值
    let maybe_number: Option<i32> = Some(10);
    // 通常可以通过match、if else等分支关键字去匹配变量
    match maybe_number {
        Some(value) => println!("变量为数字：{value}"),
        None => println!("变量不是数字"),
    }
    //unwrap_or方法设置选项默认值
    let mybe_number: Option<i32> = None; //通过None关键字设置选项值为None
    println!("{:?}", mybe_number.unwrap_or(42)); //通过option.unwrap_or设置默认值
    println!("{:?}", maybe_icecream(18));
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn raw_value() {
        let icecreams = maybe_icecream(12);
        assert_eq!(icecreams, Some(5));
    }
    #[test]
    fn check_icecream() {
        assert_eq!(maybe_icecream(0), Some(5));
        assert_eq!(maybe_icecream(9), Some(5));
        assert_eq!(maybe_icecream(18), Some(5));
        assert_eq!(maybe_icecream(22), Some(0));
        assert_eq!(maybe_icecream(23), Some(0));
        assert_eq!(maybe_icecream(24), None);
        assert_eq!(maybe_icecream(25), None);
    }
}
