fn capitalize_first(input: &str) -> String {
    let mut chars = input.chars(); //str.chars()返回迭代器
    match chars.next() {
        None => String::new(),
        Some(first) => first.to_uppercase().collect::<String>() + chars.as_str(),
    }
}
fn capitalize_words_vector(words: &[&str]) -> Vec<String> {
    //arr.iter()返回迭代器
    words.iter().map(|&word| capitalize_first(word)).collect()
}
fn capitalize_words_string(words: &[&str]) -> String {
    // arr.iter()返回迭代器
    words
        .iter()
        .map(|&word| capitalize_first(word))
        .collect::<Vec<String>>()
        .join("")
}
fn main() {
    println!("{}", capitalize_first("hello"));
    let words = vec!["hello", "world"];
    println!("{:?}", capitalize_words_vector(&words));
    println!("{:?}", capitalize_words_string(&words));
}
