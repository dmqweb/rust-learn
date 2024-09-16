#![allow(dead_code)]
fn trim_me(input: &str) -> &str {
    input.trim()
}
// &str切片.to_owned方法将字符串切片转变为一个有所有权的对象
fn compose_me(input: &str) -> String {
    input.to_owned() + " world!"
}
// replace方法用于替换字符串
fn replace_me(input: &str) -> String {
    input.to_owned().replace("cars", "balloons")
}
fn main() {}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn trim_a_string() {
        assert_eq!(trim_me("Hello!     "), "Hello!");
        assert_eq!(trim_me("  What's up!"), "What's up!");
        assert_eq!(trim_me("   Hola!  "), "Hola!");
    }
    #[test]
    fn compose_a_string() {
        assert_eq!(compose_me("Hello"), "Hello world!");
        assert_eq!(compose_me("Goodbye"), "Goodbye world!");
    }
    #[test]
    fn replace_a_string() {
        assert_eq!(
            replace_me("I think cars are cool"),
            "I think balloons are cool",
        );
        assert_eq!(
            replace_me("I love to look at cars"),
            "I love to look at balloons",
        );
    }
}
