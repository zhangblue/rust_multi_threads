use std::cell::{Cell, RefCell};
use std::sync::{Arc, Mutex, RwLock};
use std::thread;

fn main() {
    cell_example();
    refcell_example();
    mutex_example();
    mutex_poisoning_example();
    rwlock_example();
}

fn cell_example() {
    println!("cell_example");

    let cell = Cell::new(5);

    //------get 使用复制的方式将5拷贝了一份--------
    assert_eq!(cell.get(), 5); // get()方法是通过复制的方式将5拷贝出来。

    //------replace 替换原有的值--------
    assert_eq!(cell.replace(10), 5); //将之前的5替换成了10
    assert_eq!(cell.get(), 10); // 现在再拿出来就是10了

    //-------into_inner 取出了cell中的值，同时cell被消费销毁---------
    let ten = cell.into_inner();
    assert_eq!(ten, 10);
    // assert_eq!(cell.get(), 10);// 此处会报错，告知cell已经被move了

    //-------take 将之前的值取出，并放入一个默认值---------
    let cell = Cell::new(String::from("Hello"));
    assert_eq!(cell.take(), "Hello"); // 第一次take是将Hello取出来了，同时使用String的默认值放入到了Cell中
    assert_eq!(cell.take(), String::default()); // 第二次take拿到的则是第一次take后放入的默认值

    //-------set-------
    cell.set(String::from("World"));
    // cell.get() // 此处cell没有get方法，因为get()方法是使用Copy的方式将值复制，而String没有实现Copy
    let world = cell.take();
    assert_eq!(world, "World");
}

fn refcell_example() {
    println!("refcell_example");
    let rc = RefCell::new(5);
    //------- 可以允许有多个不可变引用存在-------
    {
        // 添加作用域括号，这样在作用域结束后，不可变引用则会被销毁，此时再执行borrow_mut()就不会panic
        let _five = rc.borrow();
        let _five1 = rc.borrow();
    }
    //------- 添加可变引用，在不可变一用没有添加作用域括号时，会panic ----
    let mut f = rc.borrow_mut();
    *f += 6;
    println!("drop之前：{rc:#?}"); // drop前，此时value内容被标记为<borrowed>, 表示已经被可变借用借用走了。

    let v = rc.try_borrow();
    assert!(v.is_err()); // 因此时存在了一个可变借用，所以在调用try_borrow时会返回一个Err表示无法获取不可变借用。

    drop(f);
    println!("drop后：{rc:#?}"); // 将可变借用删除后，此时value的内容则回到了5+6后的结果
}

/// 正常使用的Mutex例子
fn mutex_example() {
    println!("mutex_example");
    // ----- 正常的 Mutex 例子-----
    static NUMBERS: Mutex<Vec<u32>> = Mutex::new(Vec::new());

    let mut handles = Vec::new();
    for _ in 0..20 {
        let h = thread::spawn(|| {
            let mut lock = NUMBERS.lock().unwrap();
            lock.push(1);
        });
        handles.push(h);
    }
    handles.into_iter().for_each(|h| h.join().unwrap());

    let lock = NUMBERS.lock().unwrap();
    println!("{:?}", lock);
}

/// 中毒的Mutex例子
fn mutex_poisoning_example() {
    println!("mutex_poisoning_example");
    let data = Arc::new(Mutex::new(0));

    {
        // 线程1: 创造 panic()，让Mutex处于中毒状态
        let data = Arc::clone(&data);
        thread::spawn(move || {
            let mut lock = data.lock().unwrap();
            *lock += 1;
            panic!(); // 此处强制让第一个线程发生panic, 使Mutex处于中毒状态
        })
        .join()
        .unwrap_err();
    }

    {
        // 线程2: 从中毒的Mutex中获取数据
        let data = Arc::clone(&data);
        thread::spawn(move || match data.lock() {
            Ok(mut guard) => {
                // 因为已经处于中毒状态了，所以lock结果是Err，不会进入到Ok这里
                println!("Thread2 OK");
                *guard += 10000;
            }
            Err(poisoned) => {
                println!("Thread2 poisoned");
                let mut guard = poisoned.into_inner(); // 从中毒的Mutex中获取数据
                *guard += 1;
                println!("Thread2: New value :{}", *guard);
            }
        })
        .join()
        .unwrap();
    }
}

fn rwlock_example() {
    println!("rwlock_example");
    let counter = Arc::new(RwLock::new(0));
    let mut handles = Vec::new();

    for i in 0..10 {
        let counter = Arc::clone(&counter);
        let h = std::thread::spawn(move || {
            let value = counter.read().unwrap();
            println!("Thread {i}, value: {value}")
        });
        handles.push(h);
    }

    {
        let counter = Arc::clone(&counter);
        let h = thread::spawn(move || {
            let mut value = counter.write().unwrap();
            *value += 1;
            println!("Write updated the value to {value}")
        });
        handles.push(h);
    }

    handles.into_iter().for_each(|h| h.join().unwrap());

    println!("Counter: {}", counter.read().unwrap());
}


