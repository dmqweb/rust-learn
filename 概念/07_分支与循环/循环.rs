use std::vec;

fn main() {
    //loop无限循环
    let mut index = 0;
    let res = 'myloop: loop {
        if index == 5 {
            index += 1;
            continue 'myloop;
        } else if index == 10 {
            break 'myloop true;
        }
        println!("{index}");
        index += 1;
    };
    println!("loop循环返回结果：{res}");
    //while条件循环
    let mut count = 0;
    while count < 2 {
        println!("while循环执行：{count}次");
        count += 1;
    }
    //for in 范围循环
    for i in 3..5 {
        println!("当前数量是：{i}");
    }
    let mut arr = [1, 3, 5];
    for mut i in arr {
        i += 10;
        println!("数组中默认每一项不可变{}", i);
    }
    println!("{:?}", arr); //数组中默认每一项是不可变的，
    for i in arr.iter_mut() {
        *i += 100; //i是传递的引用，使用*来解引用
        println!("iter_mut方法获取数组每一项的可变引用{i}");
    }
}
