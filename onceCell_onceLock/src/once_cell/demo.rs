use std::cell::OnceCell;

pub fn once_cell_example_1() {
    let cell = OnceCell::new();
    assert!(cell.get().is_none());

    let result = cell.set(String::from("hello"));
    assert!(result.is_ok());

    let result = cell.set(String::from("world"));
    assert!(result.is_err());
}

pub fn once_cell_example_2() {
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