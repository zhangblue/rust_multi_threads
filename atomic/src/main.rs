use std::sync::atomic::{AtomicUsize, Ordering};
use std::thread;
use std::thread::current;
use std::time::Duration;

fn main() {
    // ordering_relaxed_example();
    compare_exchange_example();
}

/// 用于测试 Ordering::Relaxed  (只保证这个操作是原子的, 没有对排序的约束)
/// 需求：有1000个任务，分成若干个线程来干，但是要求每秒显示已完成的任务占总任务数的多少。也就是实时展示这个任务完成的进展
///
fn ordering_relaxed_example() {
    let done = AtomicUsize::new(0);

    std::thread::scope(|s| {
        // 启动干活的线程
        for _ in 0..10 {
            s.spawn(|| {
                for i in 0..100 {
                    thread::sleep(Duration::from_millis(20));
                    {
                        // 这两步骤分别获取和修改，并不是原子操作，所以最终并不是1000
                        // let current = done.load(Ordering::Relaxed);
                        // done.store(current + 1, Ordering::Relaxed);
                    }
                    // 使用fetch_add 将加载和修改编程原子操作
                    done.fetch_add(1, Ordering::Relaxed);
                }
            });
        }

        // 启动打印进度的逻辑，这个逻辑其实是在主线中运行
        loop {
            let n = done.load(Ordering::Relaxed);
            if n == 1000 {
                break;
            }
            println!("进度是: {n}/1000 已经完成");
            thread::sleep(Duration::from_secs(1));
        }
    });

    println!("所有线程都已经干完了");
}

/// 用于测试 compare_exchange. CAS循环
/// 需求：1000个线程累计计数器+1，看最后计数器是否为1000。其中假设 AtomicUsize 没有fetch_add函数，使用compare_exchange()方法，也就是CAS循环实现原子操作
fn compare_exchange_example() {
    let counter = AtomicUsize::new(0);
    thread::scope(|s| {
        for _ in 0..1000 {
            s.spawn(|| incr(&counter));
        }
    });

    println!("Counted: {}", { counter.load(Ordering::Relaxed) });
}

// 这个函数使用 compare_exchange() 函数编写CAS循环，对计数器进行替换
fn incr(counter: &AtomicUsize) {
    let mut current = counter.load(Ordering::Relaxed);
    loop {
        // 这里无限循环模拟CAS操作
        let new = current + 1;
        // 比较并更新
        match counter.compare_exchange(current, new, Ordering::Relaxed, Ordering::Relaxed) {
            Ok(_) => return, // 如果是Ok，说明赋值成功了。
            Err(v) => {
                // 如果是error，说明值已经被其它线程修改了，此时将其它线程修改后的值作为当前值，再进行下一次循环
                println!("value changed {} -> {}", current, v);
                current = v;
            }
        }
    }
}
