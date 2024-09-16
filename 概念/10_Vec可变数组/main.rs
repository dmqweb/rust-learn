fn main() {
    // 将数组变为动态数组
    let (a, v) = arr_to_vec();
    println!("Array数组:{:?}", a);
    println!("Vector动态数组:{:?}", v);
    // 数组每一项加一
    let test = vec![1, 2, 3];
    let item_add: Vec<i32> = vec_item_add(&test); //传递引用给函数内部
    println!("每一项加一{:?}", item_add);
    // 数组中添加88
    let arr = vec![1, 2, 3, 4];
    println!("添加88{:?}", vec_add_88(arr));
    // 数组中添加一项
    let arr = [1, 2, 3, 4];
    println!("数组添加一项{:?}", add_vec_item(arr, -100));
    // 创建一个可变数组
    let mut x = Vec::new();
    let y = &mut x; //同一时间只能有一个可变引用,除非变量为mut可变的
    y.push(42);
    // println!("x和y:{:?}{:?}",x, y); 报错,同一时间只能借用一次指向堆中的数据
}
fn arr_to_vec() -> ([i32; 4], Vec<i32>) {
    let a = [10, 20, 30, 40]; //数组
    let v = vec![10, 20, 30, 40]; //动态数组
    (a, v)
}
fn vec_item_add(input: &[i32]) -> Vec<i32> {
    input.iter().map(|element| element + 1).collect()
}
fn add_vec_item(arr: [i32; 4], target: i32) -> Vec<i32> {
    let mut arr: Vec<i32> = arr.to_vec();
    arr.push(target);
    arr
}
fn vec_add_88(mut arr: Vec<i32>) -> Vec<i32> {
    arr.push(88);
    arr
}
