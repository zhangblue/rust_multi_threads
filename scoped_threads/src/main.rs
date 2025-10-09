/// 将1..10000进行求和
fn main() {
    // 定义分组数
    const CHUNCK_SIZE: usize = 10;

    // 所有要计算的集合
    let numbers: Vec<u32> = (1..10000).collect();
    // 将所有的集合按照 CHUNCK_SIZE 分成10份
    let chunks = numbers.chunks(CHUNCK_SIZE);

    // 定义限定作用域的线程。主线程会等待这个作用中的所有线程运行完毕后，才会执行后面的内容
    let sum = std::thread::scope(|scope| {
        let mut handlers = Vec::new();
        for chunk in chunks {
            // 每一份只计算自己当前份的和
            let handle = scope.spawn(move || chunk.iter().sum::<u32>());
            handlers.push(handle);
        }
        // 最后再将每一份得到的总和再求和
        handlers.into_iter().map(|h| h.join().unwrap()).sum::<u32>()
    });

    println!("Sum: {sum}");
}
