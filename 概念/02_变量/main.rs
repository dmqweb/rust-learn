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
    let ref_data = &data;
    println!("{}", ref_data);
    //可变引用
    let mut data = 10;
    let ref_to_data_mut = &mut data;
    *ref_to_data_mut += 5; // *号表示解引用操作,可以使用*来获取或修改引用的实际值
    println!("The mutable reference changed the value to: {}", data);
    let numbers = [1, 2, 3, 4];
    //当使用for in遍历时,获取到的number是每一项的不可变引用
    for number in &numbers {
        //当使用时,要通过*号运算符解引用,以便获取到引用指向的值
        println!("{}", *number); // 需要解引用来获取数组中的值
    }
    //闭包(map)
    let numbers = vec![1, 2, 3, 4, 5];
    let squared: Vec<i32> = numbers.iter().map(|number| number * number).collect();
    println!("{:?}", squared);
}
