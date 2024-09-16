use std::{
    thread,
    time::{Duration, Instant},
};
fn main() {
    let mut handles = Vec::new(); //创建可变数组存储线程
    for i in 0..10 {
        //创建线程，move关键字用于：指定闭包或线程在捕获变量时采取移动语义
        // 具体就是会将闭包或线程所需的所有捕获变量的所有权转移到闭包或线程中，而不是仅仅借用他们，从而避免数据竞争
        let handle = thread::spawn(move || {
            let start = Instant::now();
            thread::sleep(Duration::from_millis(250));
            println!("Thread {i} done");
            start.elapsed().as_millis()
        });
        handles.push(handle);
    }
    let mut results = Vec::new(); //存储执行结果
    for handle in handles {
        // join方法用于等待线程完成并收集结果
        let result = handle.join().expect("线程失败");
        results.push(result);
    }
    if results.len() != 10 {
        panic!("还有线程没执行完毕");
    }
    println!();
    for (i, result) in results.into_iter().enumerate() {
        println!("线程 {i} 花费了 {result}ms");
    }
}
