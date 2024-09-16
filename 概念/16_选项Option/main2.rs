fn main() {
    let target = "rustlings";
    let optional_target = Some(target);
    //选项中匹配的和传入项相等
    if let Some(word) = optional_target {
        println!("{}", word == target);
    }
    let range = 10;
    // 创建可变数组
    let mut optional_integers: Vec<Option<i8>> = vec![None];
    // 遍历range个，向数组中推入某一项
    for i in 1..=range {
        optional_integers.push(Some(i));
    }
    println!("推入后的数组为：{:?}", optional_integers);
    let mut cursor = range;
    // 弹出数组项后判断是否是Some类型，进行while循环
    while let Some(Some(integer)) = optional_integers.pop() {
        println!("{}", integer == cursor);
        cursor -= 1;
    }
    println!("{}{:?}", cursor == 0, optional_integers);
}
