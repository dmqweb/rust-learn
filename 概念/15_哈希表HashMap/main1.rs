use std::collections::HashMap;
fn fruit_basket() -> HashMap<String, u32> {
    let mut basket = HashMap::new();
    basket.insert(String::from("香蕉"), 2);
    basket.insert(String::from("苹果"), 3);
    basket.insert(String::from("橘子"), 4);
    basket
}
fn main() {
    println!("{:?}", fruit_basket());
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn at_least_three_types_of_fruits() {
        let basket = fruit_basket();
        assert!(basket.len() >= 3);
    }
    #[test]
    fn at_least_five_fruits() {
        let basket = fruit_basket();
        assert!(basket.values().sum::<u32>() >= 5);
    }
}
