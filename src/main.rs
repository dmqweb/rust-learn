fn main() {
    hello()
}
fn hello() {
    let ch = "你好世界";
    let en = "Hello world";
    let region = [en, ch];
    for region in region.iter() {
        println!("{region}");
    }
    let arr: [i32; 3] = [1, 2, 3];
    println!("{}", arr.len());
}
