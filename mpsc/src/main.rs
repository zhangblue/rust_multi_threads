use std::sync::mpsc;
use std::thread;

type Task = Box<dyn FnOnce() + Send + 'static>;

enum Msg {
    Call(Task),
    Quite,
}

fn hello() {
    println!("hello world!");
}

fn main() {
    let (tx, rx) = mpsc::channel::<Msg>();

    let handle = thread::spawn(move || {
        while let Ok(msg) = rx.recv() {
            match msg {
                Msg::Call(task) => task(),
                Msg::Quite => {
                    println!("消费者退出!");
                    break;
                }
            }
        }
    });

    let closure = || println!("Hello from closure!");

    tx.send(Msg::Call(Box::new(hello))).unwrap();
    tx.send(Msg::Call(Box::new(closure))).unwrap();
    tx.send(Msg::Call(Box::new(|| println!("Hello from Box new"))))
        .unwrap();
    tx.send(Msg::Quite).unwrap();

    handle.join().unwrap();
}
