//在rust中,每个函数的参数都要显示指定类型,可以不指定返回类型
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
//函数中默认不允许修改原变量,需要用mut
fn vec_add_88(mut arr: Vec<i32>) -> Vec<i32> {
    arr.push(88);
    arr
}
fn main() {
    let res = call_me(3);
    println!("{}", res);
    let arr = vec![1, 2, 3];
    println!("{:?}", vec_add_88(arr));
}
