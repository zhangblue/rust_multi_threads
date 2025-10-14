use std::cell::OnceCell;
use std::sync::OnceLock;
use std::sync::atomic::{AtomicU32, Ordering};
use std::thread;

fn main() {
    once_cell_example_1();
    once_cell_example_2();
    once_lock_example_1();
    once_lock_example_2();
}

fn once_cell_example_1() {
    let cell = OnceCell::new();
    assert!(cell.get().is_none());

    let result = cell.set(String::from("hello"));
    assert!(result.is_ok());

    let result = cell.set(String::from("world"));
    assert!(result.is_err());
}

fn once_cell_example_2() {
    let mut cell = OnceCell::new();
    assert!(cell.get().is_none());

    let value = cell.get_or_init(|| String::from("Hello World"));
    assert_eq!(value, "Hello World");
    assert!(cell.get().is_some());

    // 使用get_mut修改内部的值，前提是cell必须是mut的
    if let Some(value_ref) = cell.get_mut() {
        *value_ref = String::from("你好");
    }

    assert_eq!(cell.get().unwrap(), "你好");
}

static LOCK: OnceLock<usize> = OnceLock::new();
fn once_lock_example_1() {
    assert!(LOCK.get().is_none());

    thread::spawn(|| {
        let value = LOCK.get_or_init(|| 12345);
        assert_eq!(value, &12345);
    })
    .join()
    .unwrap();

    assert_eq!(LOCK.get(), Some(&12345));
}

// 使用OnceLock实现一个链表, 只能进行追加操作
static LIST: OnceList<u32> = OnceList::new();
static COUNTER: AtomicU32 = AtomicU32::new(0);
const LEN: u32 = 1000;
fn once_lock_example_2() {
    thread::scope(|s| {
        for _ in 0..thread::available_parallelism().unwrap().get() {
            s.spawn(|| {
                // 这句while的意思是：如果 COUNTER.fetch_add(1,Ordering::Relaxed) 的结果是在0..LEN之间的数，说明匹配上了，此时就把这个数赋值给i。如果COUNTER.fetch_add(1, Ordering::Relaxed)的结果不在0..LEN之间了，此时while循环结束
                while let i @ 0..LEN = COUNTER.fetch_add(1, Ordering::Relaxed) {
                    LIST.push(i);
                }
            });
        }
    });
    for i in 0..LEN {
        assert!(LIST.contains(&i));
    }
}

struct OnceList<T> {
    data: OnceLock<T>,
    next: OnceLock<Box<OnceList<T>>>,
}

impl<T> OnceList<T> {
    const fn new() -> OnceList<T> {
        OnceList {
            data: OnceLock::new(),
            next: OnceLock::new(),
        }
    }

    fn push(&self, value: T) {
        if let Err(value) = self.data.set(value) {
            // 这里data.set失败，说明这个OnceLock已经被设置过了，所以给next进行赋值
            let next = self.next.get_or_init(|| Box::new(OnceList::new()));
            next.push(value);
        }
    }

    fn contains(&self, example: &T) -> bool
    where
        T: PartialEq,
    {
        self.data
            .get()
            .map(|item| item == example)
            .filter(|v| *v)
            .unwrap_or_else(|| {
                self.next
                    .get()
                    .map(|next| next.contains(example))
                    .unwrap_or(false)
            })
    }
}
