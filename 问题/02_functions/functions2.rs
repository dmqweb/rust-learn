// 函数的默认返回类型是：()空元组
fn call_me(num: u64) -> () {
    for i in 0..num {
        println!("Ring! Call number {}", i + 1);
    }
}

fn main() {
    call_me(3);
}
