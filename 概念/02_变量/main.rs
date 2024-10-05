fn main() {
    //定义类型变量
    let ten: i32 = 10;
    //类型自动推断
    let one = 2;
    //定义常量,全部大写,必须要指定类型,常量必须是运行前就能确定的值
    const MAX_NUM: i32 = 100_000;
    println!("{}{}{}", ten, one, MAX_NUM);
    //变量引用
    let data: i32 = 10;
    let ref_data = &data; //类型是&i32，也就i32的引用
    println!("{}", ref_data);
    //可变变量
    let mut data = 10; //可变引用的前提是变量需要是可变变量
    let ref_to_data_mut = &mut data; //引用分为可变引用和不可变引用,不可变引用可以解引用
                                     //可变引用后就不能在被引用，同一时刻只能存在一个可变引用或多个不可变引用
    *ref_to_data_mut += 5; // *号表示解引用操作,可以使用*来获取或修改引用的实际值
                           //print的过程尝试创建了data的不可变引用，注意可变引用的声明周期内不能有它的不可变引用
    println!("The mutable reference changed the value to: {}", data);
    let mut numbers = [1, 2, 3, 4];
    //当使用for in遍历时,获取到的number是每一项的不可变引用
    for mut number in numbers {
        number = 0;
        println!("{number}");
    }
    println!("可变变量不会影响原数组：{numbers:?}");
    for mut number in &numbers {
        number = &0;
        println!("{number}");
    }
    println!("不可变引用不会影响原数组{numbers:?}");
    for number in &mut numbers {
        *number = 0;
        println!("{number}");
    }
    println!("可变引用解引用会影响原数组{numbers:?}");
    //闭包(map)
    let numbers = vec![1, 2, 3, 4, 5];
    let squared: Vec<i32> = numbers.iter().map(|number| number * number).collect();
    println!("{:?}", squared);
}
