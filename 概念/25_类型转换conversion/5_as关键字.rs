// 使用as操作符可以将一种类型强制为另一种类型
fn average(values: &[f64]) -> f64 {
    let total = values.iter().sum::<f64>();
    // 使用 as 操作符将 usize 类型的 values.len() 转换为 f64 类型
    total as f64 / values.len() as f64
}
fn main() {
    let values = [3.5, 0.3, 13.0, 11.7];
    println!("{}", average(&values));
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn returns_proper_type_and_value() {
        assert_eq!(average(&[3.5, 0.3, 13.0, 11.7]), 7.125);
    }
}
