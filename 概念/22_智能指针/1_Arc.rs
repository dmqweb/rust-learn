//智能指针Arc用于在多个线程之间安全的共享
#![forbid(unused_imports)]
use std::{sync::Arc, thread}; //开启多线程
fn main() {
    // 创建一个包含从 0 到 99 的数字的向量
    let numbers: Vec<u32> = (0..100u32).collect();
    // 将向量包装在 Arc 中，以允许线程安全的共享
    let shared_numbers = Arc::new(numbers);
    let mut join_handles = Vec::new();
    // 启动 8 个线程，每个线程处理一个不同的偏移量
    for offset in 0..8 {
        // 克隆 Arc，以便每个线程可以访问共享的数字
        let child_numbers = Arc::clone(&shared_numbers);
        // 启动一个线程处理指定偏移量的求和
        let handle = thread::spawn(move || {
            // 计算每隔 8 个数字中的指定偏移量的和
            let sum: u32 = child_numbers.iter().filter(|&&n| n % 8 == offset).sum();
            println!("偏移量 {offset} 的和是 {sum}");
        });
        // 收集所有线程句柄
        join_handles.push(handle);
    }
    // 等待所有线程完成
    for handle in join_handles {
        handle.join().unwrap();
    }
}
