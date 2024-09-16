fn main() {
    println!("{}", calcate(51));
}
fn calcate(num: u32) -> u32 {
    if is_even(num) {
        num - 10
    } else {
        num - 2
    }
}
fn is_even(num: u32) -> bool {
    num % 2 == 0
}
