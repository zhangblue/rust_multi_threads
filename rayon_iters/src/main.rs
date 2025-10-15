use rayon::prelude::*;
use std::thread;
use std::time::Instant;

fn main() {
    // rayon_sum_demo();
    // rayon_find_prime_demo();
    // rayon_thread_pool_demo1();
    // rayon_thread_pool_demo2();
    rayon_thread_broadcast_demo();
    rayon_thread_pool_join_demo();
}

/// 使用rayon进行求和
fn rayon_sum_demo() {
    let nums: Vec<u64> = (0..1000000).collect();

    let sum = nums.par_iter().sum::<u64>();

    println!("{:?}", sum);
}

/// 使用rayon实现找到1000000内所有的质数
fn rayon_find_prime_demo() {
    let nums: Vec<u64> = (2..1000000).collect();
    let now = Instant::now();
    let mut primes: Vec<&u64> = nums.par_iter().filter(|&n| is_prime(*n as u32)).collect();
    let elapsed = now.elapsed();
    primes.par_sort_unstable();
    println!("{:?}", primes);
    println!("{} 毫秒找到了 {} 个质数", elapsed.as_millis(), primes.len());
}

fn is_prime(n: u32) -> bool {
    (2..=n / 2).into_par_iter().all(|x| n % x != 0)
}

/// rayon设置线程池线程的个数，以及带作用域的线程
fn rayon_thread_pool_demo1() {
    // 手动设置线程池线程的个数
    let pool = rayon::ThreadPoolBuilder::new()
        .num_threads(4)
        .build()
        .unwrap();

    let matrix = [
        vec![1, 2, 3],
        vec![4, 5, 6],
        vec![7, 8, 9],
        vec![10, 11, 12],
    ];

    // 使用rayon带作用域的线程对矩阵中的每一行求和
    pool.scope(|scope| {
        for (i, row) in matrix.iter().enumerate() {
            scope.spawn(move |_| {
                let sum: i32 = row.iter().sum();
                println!("Row {i} sum = {}", sum);
            })
        }
    });

    println!("主线程运行完毕")
}

/// rayon 演示嵌套线程池
fn rayon_thread_pool_demo2() {
    // 手动设置外部线程池
    let outer_pool = rayon::ThreadPoolBuilder::new()
        .num_threads(2)
        .build()
        .unwrap();

    // 启动外部带作用域线程
    outer_pool.scope(|scope| {
        for stage in 0..2 {
            // 外部线程启动
            scope.spawn(move |_| {
                println!("Stage {stage} started");
                // 手动设置内部线程池
                let inner_pool = rayon::ThreadPoolBuilder::new()
                    .num_threads(2)
                    .build()
                    .unwrap();
                // 启动内部线程
                inner_pool.scope(|scope| {
                    for task in 0..2 {
                        // 启动内部线程
                        inner_pool.spawn(move || {
                            println!("\t -> Inner task {task} of stage {stage}");
                        });
                    }
                });
                println!("Stage {stage} finished");
            });
        }
    });

    println!("All stages complete");
}

/// rayon 的线程广播，让广播的线程执行一样的操作
fn rayon_thread_broadcast_demo() {
    let pool = rayon::ThreadPoolBuilder::new()
        .num_threads(4)
        .build()
        .unwrap();

    pool.scope(|scope| {
        scope.spawn_broadcast(|_scope, ctx| {
            let id = ctx.index();
            println!("Thread id is {}", id);
        })
    })
}

/// rayon 线程池 join. 每次只能两个线程进行join操作，会等待这俩线程都完成后才会结束
fn rayon_thread_pool_join_demo() {
    let pool = rayon::ThreadPoolBuilder::new()
        .num_threads(4)
        .build()
        .unwrap();

    let func1 = || println!("Hello Thread 1");
    let func2 = || println!("Hello Thread 2");

    pool.join(func1, func2);
}
