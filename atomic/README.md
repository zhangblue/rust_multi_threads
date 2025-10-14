# Atomic

## 原子操作(Atomic Operations)

- Atomic: 不可分割的操作
    - 要么执行完了，要么还没发生
- 原子操作允许不同线程安全地读取和修改同一个变量
- 原子操作是并发编程的基础
    - 所有高级并发工具都是通过原子操作实现的

## 原子类型 (Atomic Type)

- 原子类型(Atomic Type)提供线程之间原始的共享内存通信机制，是其它并发类型的基础构建
- 位于`std::sync::atomic`，以`Atomic`开头
    - 例如：`AtomicBool`,`Atomicsize`,`AtomicUsize`,`AtomicI8`,`AtomicU16` ...
- 内部可变性
    - 允许通过共享引用进行修改(例如 `&AtomicUsize`)
- 相同的接口：
    - 加载与存储 (`load`/`store`)
    - 获取并修改 (`fetch-modify`)
    - 比较和交换 (`compare-exchange`)

## Load & Store (加载 & 存储)

- `load`: 以原子方式加载原子变量中存储的值
    - `pub fn load(&self, order:Ordering)->usize`
- `store`: 以原子方式将新值存入变量
    - `pub fn store(&self, val:usize, order:Ordering)`

## Atomic Memory Ordering (原子内存排序)

- 内存排序用于指定原子操作如何同步内存

```rust
#[non_exhaustive]
pub enum Ordering {
    Relaxed,
    Release,
    Acquire,
    AcqRel,
    SeqCst,
}
```

- `Relaxed` -- 最弱保证
    - 含义：只保证这个操作是原子的
        - 没有对排序的约束
    - 适合场景：只关心“计数正确”，而不关心不同线程之间的操作顺序(例如：一个全局统计计数器，线程只需要把值加一)

## Fetch_Modify (获取 & 修改)

方法：`fetch_add`,`fetch_and`,`fetch_max`,`fetch_min`,`fetch_nand`,`fetch_or`,`fetch_sub`,`fetch_update`,`fetch_xor`

- fetch_*modify*
    - 修改原子变量
    - 同时获取(fetch)其原始值
    - 整个过程是单一的原子操作

## Compare_Exchange (比较和交换)

```rust
pub fn compare_exchange(
    &self,
    current: usize,
    new: usize,
    success: Ordering,
    failure: Ordering,
) -> Result<usize, usize> {}
```

`compare_exchange`: 会检查原子值是否等于给丁的值(current参数)

- 如果相等，就用新值(new参数)替换它，否则不做任何修改
- 整个过程是单一的原子操作
- 返回之前的值，并告知我们是否替换成功

为什么说`compare_exchange`很强大

- 你可以用它来实现几乎所有其它原子操作，比如`fetch_add`,`fetch_sub`等
- 只需在一个循环中不断尝试，直到成功为止
    - 这种模式叫做`CAS loop` (Compare-And-Swap循环)

### A->B->A问题

- 如果原子值从A编程B，又变回A，在调用`compare_exchange`之前，是无法察觉这个变化的
    - 虽然值看起来没变化，但是它确实经历了变化
    - 这在某些算法中可能导致严重问题，例如涉及原子指针的复杂算法

## compare_exchange_weak (更轻量但可能失败)

- 即使值匹配上了，`compare_exchange_weak` 版本也可能返回失败(Err),即所谓的"伪失败(spurious failure)"
- 这样可以在某些平台上更高效
- 如果失败的代价不大(例如简单的重试循环),Rust官方推荐优先使用`compare_exchange_weak`版本