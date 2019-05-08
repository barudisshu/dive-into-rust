//!
//! Rust中，处理多线程共享变量，还提供了另外一种通信方式：`mpsc`
//!
//! Multi-producer, single-consumer FIFO queue，多生产者单消费者先进先出队列
//!
//! 这种通信方式是在线程之间建立一个通信“管道”，一边发送消息，一边接收消息，完成通信
//!
//!

///
/// 异步管道
///
///
///
#[test]
fn _29_01_01_pipe() {
    use std::thread;
    use std::sync::mpsc::channel;

    let (tx, rx) = channel();
    thread::spawn(move || {
        for i in 0..10 {
            tx.send(i).unwrap();
        }
    });

    while let Ok(r) = rx.recv() {
        println!("received {}", r);
    }
}

///
///
/// 多发送端，单接收端
///
#[test]
fn _29_01_02_pipe() {
    use std::thread;
    use std::sync::mpsc::channel;

    let (tx, rx) = channel();

    for i in 0..10 {
        let tx = tx.clone();    // 复制一个新的tx，将这个复制的变量move进子线程
        thread::spawn(move || {
            tx.send(i).unwrap();
        });
    }
    drop(tx);

    while let Ok(r) = rx.recv() {
        println!("received {}", r);
    }
}

///
/// 同步管道
///
/// 同步管道的特点是：如果缓冲区被填满了，继续调用send方法的时候会发生阻塞，
/// 等待接收端把缓冲区内的消息拿走才能继续发送。
///
/// 缓冲区的长度可以在建立管道的时候设置，而且0是有效数值。
///
#[test]
fn _29_02_01_sync_pipe() {
    use std::thread;
    use std::sync::mpsc::sync_channel;

    let (tx, rx) = sync_channel(1);
    tx.send(1).unwrap();
    println!("send first");
    thread::spawn(move || {
       tx.send(2).unwrap();
        println!("send second");
    });

    println!("receive first {}", rx.recv().unwrap());
    println!("receive second {}", rx.recv().unwrap());
}