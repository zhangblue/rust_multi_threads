# Conditional Variable (条件变量)

## Condvar

在rust中是这个名字，被简写了

- 条件变量`Condition Variable`
    - 提供在等待事件发生时阻塞线程的能力
- `Condvar`表示能够阻塞一个线程的能力，使其在等待事件发生时不消耗CPU事件
- 条件变量通常与一个bool(predicate,一个条件)和一个`mutex`关联
    - 在决定线程必须阻塞之前，这个bool总是在`mutex`内部被验证
- 注意：任何试图在同一个`CondVar`上使用多个`mutex`的操作，可能会导致运行时的`panic`