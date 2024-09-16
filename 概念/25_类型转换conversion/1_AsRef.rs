// fn用于函数签名，where关键字用于指定泛型参数T的trait bounds（特征约束）
// AsRef泛型也就是说T类型的值，可以通过as_ref()方法转换为&str类型的引用
fn byte_counter<T>(arg: T) -> usize
where
    T: AsRef<str>,
{
    arg.as_ref().as_bytes().len()
}
// AsRef是标准库中的一个trait，用于将某个类型转为另一种类型的引用。
fn char_counter<T>(arg: T) -> usize
where
    T: AsRef<str>,
{
    arg.as_ref().chars().count()
}
fn num_sq<T>(arg: &mut T)
where
    T: AsMut<u32>,
{
    let val = arg.as_mut();
    *val = *val * *val;
}
fn main() {}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn different_counts() {
        let s = "Café au lait";
        assert_ne!(char_counter(s), byte_counter(s));
    }
    #[test]
    fn same_counts() {
        let s = "Cafe au lait";
        assert_eq!(char_counter(s), byte_counter(s));
    }
    #[test]
    fn different_counts_using_string() {
        let s = String::from("Café au lait");
        assert_ne!(char_counter(s.clone()), byte_counter(s));
    }
    #[test]
    fn same_counts_using_string() {
        let s = String::from("Cafe au lait");
        assert_eq!(char_counter(s.clone()), byte_counter(s));
    }
    #[test]
    fn mut_box() {
        let mut num: Box<u32> = Box::new(3);
        num_sq(&mut num);
        assert_eq!(*num, 9);
    }
}
