# 消息传递 - MPSC

Multi-producer, single-consumer FIFO 队列通信原语

- 基于消息的通道(channel)通信机制
- 三种类型
    - `Sender`
    - `SyncSender`
    - `Receiver`

## 异步通道(asynchronous channel)，无限缓冲区的通道

- `channel` 函数返回一个(`Sender`,`Receiver`)
- 发送操作是异步的
- 概念上拥有无限容量的缓冲区

## 同步通道(synchronous channel)，有界缓冲区的通道

- `sync_channel`函数返回一个(`SyncSender`,`Receiver`)
- 其内部为预先分配好的固定大小的缓冲区
- 发送操作都是同步的，除非缓冲区还有容量，否则就阻塞
- 注意：缓冲区大小允许设置为0，通道变成一种 "会合(rendezvous)通道"
    - 每个发送者都会原子的将消息直接交给接收者

## 断开链接(Disconnection)

- 对通道的`send`和`receive`操作都会返回一个`Result`，用以指示操作是否成功
    - 如果操作失败，通常表示通道的另一端已经"挂起"或被释放(drop)
- 一旦通道的一半被释放，大多数操作将无法继续运行，因此会返回Err
    - 在许多应用中，开发者会继续对这些`Result`调用`unwarp`，从而在某个线程意外终止时，引发错误在其它线程中传播