fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

fn main() {
    let string1 = String::from("字符串1");
    let result;
    {
        let string2 = String::from("字符串2");
        //这里&string2会报错，因为longest会返回一个引用，在string2声明的外部获取不到&string2的引用
        result = longest(&string1, &string2);
    }
    println!("The longest string is '{result}'");
}
