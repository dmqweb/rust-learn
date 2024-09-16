use core::cell::{Cell, RefCell};
use std::collections::{BTreeMap, BTreeSet, HashMap, HashSet, VecDeque};
use std::rc::Rc;
use std::sync::atomic::{AtomicI32, Ordering};
use std::sync::{Arc, Mutex, RwLock};

fn main() {
    //整数
    let _i8: i8 = -128; //有符号8位整数(-128-127) , 同理64、128
    let _i8_ = 127i8;
    let _u8: u8 = 0; //无符号8位整数(0-255) , 同理64、128
    let _u8_ = 255u8;
    let _isize: isize = 10; //有符号指针大小整数,大小取决于目标平台的指针大小(64位平台其范围为-2^63到2^63)
    let _isize_ = 10isize;
    let _usize: usize = 10;
    let _usize_ = 10usize; //无符号指针大小整数(64位平台上,其范围为:0到2^64-1)

    //浮点数
    let _f32: f32 = f32::MIN; //最大值和最小值
    let _f64: f64 = f64::MAX;

    //字符
    let _str: &str = "hello world"; //""用于定义字符串字面量切片(多个字符),类型为&str字符串切片类型
    let _char: char = 'A'; //''用于声明单个字符串,类型为char
    let _str = String::from("Hello Rust"); //String UTF-8字符串类型

    // 元组
    let _tuple: (i32, &str, char) = (1, "hello", 'r');

    //数组
    let _arr: [i32; 3] = [1, 2, 3]; //数组类型必须一致,类型定义的首项为类型,尾项为个数
    let _vec_arr: Vec<i32> = vec![1, 2, 3, 4]; //动态大小的数组
    let _init_arr = [0; 100]; //数组赋值使用;代表批量赋值到某长度
    let _arr_slice = &_arr[1..2]; //数组索引切片,不包含尾

    //双端队列
    let mut deque: VecDeque<i32> = VecDeque::new();
    deque.push_back(1);
    deque.push_front(2);
    println!("The deque contains: {:?}", deque);

    //结构体
    struct Point {
        //结构体是聚合的类型,通过impl实现方法后就类似于对象
        x: i32,
        y: i32,
    }
    let _point = Point { x: 10, y: 20 };

    //枚举
    enum Message {
        //枚举类型中定义结构体时,必须写没有定义过的结构体
        Char(char),
        Int(i32),
        Person { name: char },
    }
    let _message = Message::Person { name: 'A' };
    let _message_ = Message::Int(100);

    //元组结构体
    struct Color(i32, i32, i32);
    let color = Color(255, 255, 255);
    println!("{}{}{}", color.0, color.1, color.2);

    //Option<T> 值可能存在可能不存在
    let _option_value: Option<i32> = Some(10);
    let _option_: Option<_> = Some(10);

    //Result<T,E> 操作可能成功或者失败
    fn divide(a: f64, b: f64) -> Result<f64, &'static str> {
        if b == 0.0 {
            Err("Cannot divide by zero")
        } else {
            Ok(a / b)
        }
    }
    let result = divide(10.0, 2.0);
    println!("The result is: {:?}", result);

    //Box<T>智能指针(指向堆上的数据)
    let _box_data = Box::new(10);

    //Rc<T>引用计数类型(允许有多个所有者)
    let data = Rc::new(10);
    let another = Rc::clone(&data);
    println!("两者相等: {}", data == another);

    //Arc<T>线程安全的引用计数类型(允许有多个所有者)
    let data = Arc::new(10);
    let another = Arc::clone(&data);
    println!("两者相等: {}", data == another);

    //HashMap<K,V>映射
    let mut map: HashMap<&str, i32> = HashMap::new();
    map.insert("key1", 10);
    println!("map集合中有:{:?}", map);

    //HashSet<V>去重集合
    let set: HashSet<i32> = HashSet::from([3, 1, 4, 2]);
    println!("set集合中有:{:?}", set);

    //BTreeMap<K,V>有序的映射
    let mut btree_map: BTreeMap<&str, i32> = BTreeMap::new();
    btree_map.insert("key1", 10);
    println!("BTreeMap集合中有:{:?}", btree_map);

    //BTreeSet<V>有序的集合
    let btree_set: BTreeSet<i32> = BTreeSet::from([3, 2, 1]);
    println!("BTreeSet集合中有:{:?}", btree_set);

    //Cell<V>可变借用检查的不可变类型
    let cell = Cell::new(10);
    println!("cell为:{}", cell.get());
    cell.set(20);
    println!("cell改变:{}", cell.get());

    //RefCell<V>内部可变性
    let refcell = RefCell::new(10);
    *refcell.borrow_mut() += 5;
    println!("The RefCell now contains: {}", refcell.borrow());

    //Mutex<V>线程安全的共享可变性
    let mutex = Mutex::new(10);
    println!("The mutex contains: {}", *mutex.lock().unwrap());

    //RwLock<V>读写锁,允许多个读取者或者一个写者
    let rwlock = RwLock::new(10);
    println!("The RwLock contains: {}", *rwlock.read().unwrap());

    //Atommic* 线程安全的原子类型
    let atomic = AtomicI32::new(10);
    println!("The atomic contains: {}", atomic.load(Ordering::SeqCst));
    atomic.store(20, Ordering::SeqCst);
    println!("The atomic now contains: {}", atomic.load(Ordering::SeqCst));

    //fn(T)函数类型,用于函数指针或者闭包
    let fn_ptr: fn(i32) -> i32 = |x| x + 1;
    println!("函数的执行结果是:{}", fn_ptr(5));

    //&dyn类型，用于指定trait对象的引用
    trait Licensed {}
    fn test(arg1: &dyn Licensed) -> bool {
        true
    }
}
