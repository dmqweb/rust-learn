use std::{
    sync::{Arc, Mutex},
    thread,
    time::Duration,
};
struct JobStatus {
    jobs_done: u32,
}
fn main() {
    // 使用 Arc 和 Mutex 组合来在线程中共享和修改状态
    let status = Arc::new(Mutex::new(JobStatus { jobs_done: 0 }));
    let mut handles = Vec::new();
    for _ in 0..10 {
        let status_shared = Arc::clone(&status);
        let handle = thread::spawn(move || {
            thread::sleep(Duration::from_millis(250));
            // 锁定 Mutex，然后更新共享值
            let mut status = status_shared.lock().unwrap();
            status.jobs_done += 1;
        });
        handles.push(handle);
    }
    // 等待所有线程完成
    for handle in handles {
        handle.join().unwrap();
    }
    // 打印结果
    let status = status.lock().unwrap();
    println!("Jobs done: {}", status.jobs_done);
}
