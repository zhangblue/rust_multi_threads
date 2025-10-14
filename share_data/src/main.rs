use std::sync::Arc;
use std::thread;

fn main() {
    share_data_box();
    share_data_arc();
}

fn share_data_box() {
    let data: &'static [i32; 5] = Box::leak(Box::new([1, 2, 3, 4, 5]));

    let mut handles = Vec::new();

    for _ in 0..100 {
        let handle = thread::spawn(move || unsafe {
            println!("Data: {data:?}");
        });
        handles.push(handle);
    }

    handles.into_iter().for_each(|h| h.join().unwrap());
}

fn share_data_arc() {
    let data = Arc::new([1, 2, 3, 4, 5]);

    let mut handles = Vec::new();

    for _ in 0..100 {
        let local_data = data.clone();
        let handle = thread::spawn(move || {
            println!("Data: {local_data:?}");
        });
        handles.push(handle);
    }

    handles.into_iter().for_each(|h| h.join().unwrap());
}
