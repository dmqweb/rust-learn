fn main() {
    //原始字符串引用(不用转义)
    let s = r#"C:\Program Files\SomeProgram"#;
    println!("{}", s);
    //变量默认不可变,要使用mut
    let mut x = 5;
    println!("这个值是:{}", x);
    x = 6;
    println!("值变为了:{}", x);
    //使用_开头命名就可以不使用该变量
    let _x = 5;
    //变量的解构
    let (a, mut _b): (bool, bool) = (true, true);
    println!("a:{:?},b:{:?}", a, _b);
    assert_eq!(a, _b);
    _b = false;
    // assert_eq!(a, b);   //断言报错
    //解构赋值
    struct Struct {
        //定义结构体
        e: i32,
    }
    let (a, b, c, d, e);
    (a, b) = (1, 2);
    //解构赋值省略部分项
    [c, .., d, _] = [1, 2, 3, 4, 5];
    Struct { e, .. } = Struct { e: 5 };
    assert_eq!([1, 2, 1, 4, 5], [a, b, c, d, e]);
    //定义常量,必须全部大写
    const _MAX_POINTS: u32 = 100_000; //数字字面量可提高可读性
                                      //变量遮蔽
    let x = 5;
    let x = x + 1;
    {
        let x = x * 2;
        println!("块级作用域遮蔽:{}", x);
    }
    println!("遮蔽之后的变量为:{}", x);
    //如果无需再使用之前的变量,就可以使用重复的变量进行 遮蔽
    let spaces = "       ";
    let spaces = spaces.len();
    println!("{}", spaces);
    //不允许不同类型间赋值
    let mut _str = "    ";
    // _str = str.len(); 报错
    //直接打印变量
    let char_str = '符';
    let string_str = "字符串类型";
    let arr = [1, 2];
    println!("{char_str},{string_str}");
    println!("{arr:?}");
    type A = [i32; 3]; //type关键字定义新类型
    let a_instance: A = [1, 2, 3];
    println!("{a_instance:?}");
}
