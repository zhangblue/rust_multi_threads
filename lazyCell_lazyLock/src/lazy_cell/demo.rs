use std::cell::LazyCell;

pub fn lazy_cell_init_demo() {
    let lazy = LazyCell::<i32>::new(init);
    println!("--------");
    //第一次获取值时，才会调用init函数进行初始化
    println!("{}", *lazy);
    // 第二次获取值时就不会指定init函数了
    println!("{}", *lazy);
}

fn init() -> i32 {
    println!("LazyCell initializing....");
    23
}


