use std::{sync::mpsc, thread, time::Duration};
struct Queue {
    first_half: Vec<u32>,
    second_half: Vec<u32>,
}
impl Queue {
    fn new() -> Self {
        Self {
            first_half: vec![1, 2, 3, 4, 5],
            second_half: vec![6, 7, 8, 9, 10],
        }
    }
}
fn send_tx(q: Queue, tx: mpsc::Sender<u32>) {
    let tx1 = tx.clone(); // 克隆发送端
    thread::spawn(move || {
        for val in q.first_half {
            println!("Sending {val:?}");
            tx1.send(val).unwrap();
            thread::sleep(Duration::from_millis(250));
        }
    });
    let tx2 = tx.clone(); // 再次克隆发送端
    thread::spawn(move || {
        for val in q.second_half {
            println!("Sending {val:?}");
            tx2.send(val).unwrap();
            thread::sleep(Duration::from_millis(250));
        }
    });
}
fn main() {
    // You can optionally experiment here.
    let (tx, rx) = mpsc::channel();
    let queue = Queue::new();
    send_tx(queue, tx);
    let mut received = Vec::with_capacity(10);
    for value in rx {
        received.push(value);
    }
    received.sort();
    assert_eq!(received, [1, 2, 3, 4, 5, 6, 7, 8, 9, 10]);
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn threads3() {
        let (tx, rx) = mpsc::channel();
        let queue = Queue::new();
        send_tx(queue, tx);
        let mut received = Vec::with_capacity(10);
        for value in rx {
            received.push(value);
        }
        received.sort();
        assert_eq!(received, [1, 2, 3, 4, 5, 6, 7, 8, 9, 10]);
    }
}
