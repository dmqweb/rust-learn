fn main() {
    //范围运算符.. , 不包含末尾
    for i in 0..10 {
        println!("{}", i);
    }
    //获取数组引用的子切片
    let arr = [1, 2, 3, 4, 5];
    let arr_slice = &arr[1..4];
    println!("{}", arr_slice.len());
}
