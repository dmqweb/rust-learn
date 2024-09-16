fn main() {
    //执行多国输出语句
    greet_world();
}
fn greet_world() {
    let english = "world hello";
    let chinese = "你好世界";
    let regions = [english, chinese];
    //使用for in遍历数组,集合类型需要通过.iter进行迭代循环
    for region in regions.iter() {
        // 输出region项
        println!("{}", region);
    }
    let arr: [i32; 3] = [1, 2, 3];
    //println!宏的格式化占位符有: {}默认格式、{:?}调试格式、{:#?}美化的调试格式、
    // {:.2}留两位小数、{:20}字符串最大宽度20、{0}第一个参数、{variable}变量不用传递
    println!("{}", arr.len());
    println!("{:?}", arr);
    println!("{:#?}", arr);
    println!("{1}---{0}", "参数1", "参数2");
    println!("{name}已经{age}岁了", name = "卡其", age = 3);
    let (name, age) = ("卡其小猫", 3);
    println!("{name}已经{age}岁了");
}
