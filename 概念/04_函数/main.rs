fn main() {
    let res = call_me(3);
    println!("{}", res);
    let arr = vec![1, 2, 3];
    println!("{:?}", vec_add_88(arr)); //传递所有权之后就不能在使用
                                       // 可变引用的前提是变量是可变的
    let mut arr = vec![4, 5, 6];
    println!("{:?}", vec_add_66(&mut arr));
}
//在rust中,每个函数的参数都要显示指定类型,返回类型默认是()
fn call_me(num: i32) -> i32 {
    //可变变量
    let mut res: i32 = 0;
    for i in 0..num {
        res += i;
        println!("{}", i + 1);
    }
    //注意在rust中分号表示结束表达式或语句分割或宏调用,最后一个表达式最为返回值时不应该加分号
    res //函数的最后一个表达式就是返回值
}
//函数可以接收变量的所有权，并设置变量的可变性，rust中变量的可变性是由变量的借用状态决定的，而不是变量本身状态决定的
fn vec_add_88(mut arr: Vec<i32>) -> Vec<i32> {
    arr.push(88);
    arr
}
fn vec_add_66(arr: &mut Vec<i32>) -> &mut Vec<i32> {
    arr.push(66);
    arr
}
