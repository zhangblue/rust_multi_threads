use std::sync::{Arc, Condvar, Mutex};
use std::thread;

/// 目的：使用条件变量，让主线程等待子线程完成某个动作，并且在等待的时候不消耗CPU
fn main() {
    let pair = Arc::new((Mutex::new(false), Condvar::new()));
    let par2 = Arc::clone(&pair);

    // 启动子线程
    thread::spawn(move || {
        let (lock, cvar) = &*par2;
        let mut started = lock.lock().unwrap();
        *started = true;
        cvar.notify_one(); // 唤醒一个等待这个条件的线程。这里唤醒的是主线程
    });

    // 这里是主线程逻辑
    let (lock, cvar) = &*pair;
    let mut started = lock.lock().unwrap();
    while !*started {
        started = cvar.wait(started).unwrap(); // 主线程等待条件变量的唤醒。这里等待的是子线程中的notify_one()唤醒
    }
}
