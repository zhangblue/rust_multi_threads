fn main() {
   let handle =  std::thread::Builder::new()
        .name("Thread 1".to_owned()) // 设置线程的名字
        .stack_size(4 * 1024 * 1024) // 设置线程的栈大小
        .spawn(another_thread)// 生成一个线程。这里调用了一个函数
        .unwrap();

    handle.join().unwrap();
}

fn another_thread() {
    println!("In threat: {}", std::thread::current().name().unwrap());
}
