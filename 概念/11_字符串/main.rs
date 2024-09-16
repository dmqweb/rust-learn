fn main() {
    let data = "Rust语言";
    println!("{}", data);
    let data = data.to_string();
    println!("{}", data);
    println!("最后一个字符:{}", get_char(&data));
    println!("大写{}", string_uppercase("abcd"));
    println!("字符截取{}", string_slice("123456", 1, 4));
    let answer = current_favorite_color();
    println!("最爱的颜色: {answer}");
    let word = String::from("green");
    //字符串引用，函数返回布尔值
    if is_a_color_word(&word) {
        println!("That is a color word I know!");
    } else {
        println!("That is not a color word I know.");
    }
}
fn get_char(data: &str) -> char {
    data.chars().last().unwrap()
}
//String代表不定长的堆字符串集合类型,&str代表字符串切片类型,固定长度,char代表单个字符
fn string_uppercase(data: &str) -> String {
    data.to_uppercase()
}
fn string_slice(str: &str, start: usize, end: usize) -> String {
    //字符串截取
    let bytes = str.as_bytes();
    let slice = &bytes[start..end];
    //unwrap方法将提取OK成员,panic错误成员
    String::from_utf8(slice.to_vec()).unwrap()
}
fn current_favorite_color() -> &'static str {
    "blue"
}
fn is_a_color_word(attempt: &str) -> bool {
    attempt == "green" || attempt == "blue" || attempt == "red"
}
