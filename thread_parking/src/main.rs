use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
use std::thread;
use std::time::Duration;

fn main() {
    let flag = Arc::new(AtomicBool::new(false));
    let flag2 = Arc::clone(&flag);

    // 启动了一个线程，用于检查flag2如果是true，则结束这个线程
    let parked_thread = std::thread::spawn(move || {
        // 检查标记，这个标记在主线程中被修改成true后，挂起线程在恢复后，能够获取到正确的值
        while !flag2.load(Ordering::Relaxed) {
            println!("Parking thread");
            thread::park(); // 挂起当前线程
            println!("Thread unparked");
        }
        println!("Flag received");
    });

    // 主线程睡100毫秒
    thread::sleep(Duration::from_millis(100));
    flag.store(true, Ordering::Relaxed); // 主线程中修改flag的值
    parked_thread.thread().unpark(); // 在主线程中将挂起的线程恢复。此时恢复后的线程能够获取到主线程修改后的值
    parked_thread.join().unwrap(); // 等待parked线程运行完毕
}
