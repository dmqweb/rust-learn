use core::panic;
use std::{num::ParseIntError, ops::Not};

fn main() {
    //数字次方
    let base: i32 = 2;
    let expoent = 3;
    println!("{}", base.pow(expoent));
    //布尔取反
    let is_morning = true;
    // let is_evening = !is_morning;
    let is_evening = is_morning.not();
    println!("{}{}", is_morning, is_evening);
    //char字符判断类型
    let str = '2';
    if str.is_alphabetic() {
        println!("Alphabetical类型");
    } else if str.is_numeric() {
        println!("Numberical类型");
    } else {
        println!("其他类型");
    }
    //数组的len返回长度
    let a = [0; 100]; //数组赋值使用;代表批量赋值到长度
    if a.len() >= 100 {
        println!("数组长度大于100");
        println!("数组为: {:?}", a);
    } else {
        println!("数组长度很小");
        panic!("数组长度太小了,重新赋值");
    }
    //数组切片 &arr[start..end]
    let a = [1, 2, 3, 4, 5];
    let nice_slice = &a[1..4]; //从索引1到4进行切片,不包含尾
    println!("{:?}", nice_slice);
    //数组最大值
    let numbers: [i32; 5] = [1, 2, 3, 4, 5];
    println!("最大值为:{}", numbers.iter().max().unwrap());
    //解构元组
    let cat = ("卡其小猫", 3.5);
    let (name, age) = cat;
    println!("{name} 已经 {age} 岁了");
    // 元组索引访问
    let numbers = (1, 2, 3);
    let second = numbers.1;
    println!("{}", second == 2);
}
