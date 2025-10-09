# Thread Builder 模式

## 线程命名
可以使用`std::thread::Builder`来构建一个带名字的线程
- 如果一个命名线程发生 panic，Rust会在 panic 报错信息中打印出该线程的名字，方便快速定位问题
- 在线程运行时，他的名字也会被提供给操作系统，比如在类 Unix 系统中会使用 `pthread_setname_np` 设置线程名
- `Builder::name()`

## 线程栈大小 (Stack Size)
Rust中每个线程都会有一个默认的栈空间大小，这个大小时平台相关的
- 当前默认设置：
  - 目前在所有 Tier-1 平台(Rust 官方支持的主平台)上，默认线程栈大小是2MiB (约2兆字节)

## 设置线程栈的大小
- `Builder::stack_size()`
- 设置环境变量 `RUST_MIN_STACK`
  - 例如：`RUST_MIN_STACK=4194304 ./your_program`
  - 注意：如果你在代码中同时使用了`Builder::stack_size`, 那么它会覆盖`RUST_MIN_STACK`的设置
- 主线程(main函数所在的线程)的栈大小不是由Rust决定的，而是操作系统或者启动器控制的
