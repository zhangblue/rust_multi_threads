use std::sync::LazyLock;
use std::thread;

static NUMBER: LazyLock<i32> = LazyLock::new(|| {
    println!("LazyLock initializing....");
    100
});

/// 启动多线程访问 LazyLock 中的值。
pub fn lazy_lock_init_demo() {
    let handles: Vec<_> = (0..5)
        .map(|i| {
            thread::spawn(move || {
                println!("Thread {i} 看到的Number中的数是: {}", *NUMBER);
            })
        })
        .collect();

    handles.into_iter().for_each(|h| h.join().unwrap());
}
