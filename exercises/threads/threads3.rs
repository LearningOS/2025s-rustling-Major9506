// threads3.rs

use std::sync::mpsc;
use std::sync::Arc;
use std::sync::Mutex;
use std::thread;
use std::time::Duration;

struct Queue {
    length: u32,
    first_half: Vec<u32>,
    second_half: Vec<u32>,
}

impl Queue {
    fn new() -> Self {
        Queue {
            length: 10,
            first_half: vec![1, 2, 3, 4, 5],
            second_half: vec![6, 7, 8, 9, 10],
        }
    }
}

fn send_tx(q: Queue, tx: mpsc::Sender<u32>) -> Vec<thread::JoinHandle<()>> {
    let qc = Arc::new(q);
    let qc1 = Arc::clone(&qc);
    let qc2 = Arc::clone(&qc);

    let tx1 = tx.clone();
    let handle1 = thread::spawn(move || {
        for val in &qc1.first_half {
            println!("sending {:?}", val);
            tx1.send(*val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    let tx2 = tx.clone();
    let handle2 = thread::spawn(move || {
        for val in &qc2.second_half {
            println!("sending {:?}", val);
            tx2.send(*val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    vec![handle1, handle2]
}

fn main() {
    let (tx, rx) = mpsc::channel();
    let queue = Queue::new();
    let queue_length = queue.length;

    let handles = send_tx(queue, tx);

    // 使用 Arc 和 Mutex 来共享和修改 total_received
    let total_received = Arc::new(Mutex::new(0));
    let total_received_clone = Arc::clone(&total_received);

    // 使用一个线程来接收消息
    let handle_rx = thread::spawn(move || {
        for received in rx {
            println!("Got: {}", received);
            let mut count = total_received_clone.lock().unwrap();
            *count += 1;
        }
    });

    // 等待所有发送线程完成
    for handle in handles {
        handle.join().unwrap();
    }

    // 等待接收线程完成
    handle_rx.join().unwrap();

    // 获取 total_received 的值
    let final_count = total_received.lock().unwrap();
    println!("total numbers received: {}", *final_count);
    assert_eq!(*final_count, queue_length);
}