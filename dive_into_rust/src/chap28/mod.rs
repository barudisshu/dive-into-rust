//!
//!
//! 多线程
//!
//!

///
///
///
#[test]
fn _28_01_01_thread() {
    use std::thread;

    thread::spawn(move || {
       // 这里是新建线程的执行逻辑
    });
}

///
/// 如果需要等待子线程执行结束，可以使用join方法
///
#[test]
fn _28_01_02_thread() {
    use std::thread;
    // child 的类型 `JoinHandle<T>`，这个T是闭包的返回类型
    let child = thread::spawn(move || {
        // 子线程的逻辑
    });
    // 父线程等待子线程结束
    let res = child.join();
}

///
/// 如果需要为子线程指定更多的参数信息，那么在创建的时候可以使用Builder模式
///
#[test]
fn _28_01_03_thread() {
    use std::thread;

    thread::Builder::new().name("child1".to_string()).spawn(move || {
        println!("Hello, world!");
    });
}

///
/// thread模块还提供了下面几个工具函数
///
/// - `thread::sleep(dur: Duration)` 使得当前线程等待一段时间继续执行。在等待时间内，线程调度会调度其它的线程来执行。
/// - `thread::yield_now()` 放弃当前线程的执行，要求线程调度器执行线程切换。
/// - `thread::current()` 获得当前的线程。
/// - `thread::park()` 暂停当前线程，进入等待状态。当`thread::Thread::unpark(&self)`方法被调用的时候，这个线程可以被恢复执行。
/// - `thread::Thread::unpark(&self)` 恢复一个线程的执行。
///
#[test]
fn _28_01_04_thread() {
    use std::thread;
    use std::time::Duration;

    let t = thread::Builder::new()
        .name("child1".to_string())
        .spawn(move || {
            println!("enter child thred.");
            thread::park();
            println!("resume child thread");
        }).unwrap();
    println!("spwan a thread");
    thread::sleep(Duration::new(5, 0));
    t.thread().unpark();
    t.join();
    println!("child thread finished");
}

///
/// 免数据竞争
///
#[test]
fn _28_03_01_data_race() {
    use std::thread;
    let mut health = 12;
    thread::spawn(move || {
        health *= 2;
    });
    println!("{}", health);
    // 修改无效，因为health是Copy trait的，闭包内实际上是一个新的拷贝
}

///
/// Rust是如何实现免疫数据竞争的。主要是两个特殊的trait.
///
/// - `std::marker::Sync`
/// - `std::marker::Send`
///
/// ```
/// pub fn spawn<F, T>(f: F) -> JoinHandle<T>
///     where F: FnOnce() -> T, F: Send + 'static, T: Send + 'static
/// ```
///
/// 参数类型`F`有重要的约束条件`F:Send + 'static, T: Send + 'static`。但凡在线程间传递所有权都会发生安全问题的类型，
/// 都无法在这个参数中出现，否则就是编译错误。
///
#[test]
fn _28_04_01_sync() {

}

///
/// `Arc`
///
/// Arc是Rc的线程安全版本。全称“Atomic reference counter”。
/// 它跟Rc最大的区别在于，引用计数用的是源自整数类型。Arc使用方法示例如下，
///
#[test]
fn _28_05_01_arc() {
    use std::sync::Arc;
    use std::thread;

    let numbers: Vec<_> = (0..100u32).collect();
    // 引用计数指针，指向一个Vec
    let shared_numbers= Arc::new(numbers);
    // 循环创建10个线程
    for _ in 0..10 {
        // 复制引用计数指针，所有的Arc都指向一个Vec
        let child_numbers = shared_numbers.clone();
        // move修饰闭包，上面这个Arc指针被move进入了新线程中
        thread::spawn(move || {
            // 我们希望可以在新线程中使用Arc，读取共享的那个Vec
            let local_numbers = &child_numbers[..];
            // 继续使用Vec中的数据
        });
    }
}

///
/// 多线程修改共享变量
///
/// 使用Arc来实现多线程之间的共享，使用Mutex来提供内部可变性。每次需要修改的时候，
/// 我们需要调用`lock()`方法（或者try_lock）获得锁，然后才能对内部的数据进行读/写操作。
/// 因为锁的存在，我们就可以保证整个“读/写”是一个完整的transaction。
///
#[test]
fn _28_05_02_thread_shard() {
    use std::sync::Arc;
    use std::sync::Mutex;
    use std::thread;

    const COUNT: u32 = 1_000_000;

    let global = Arc::new(Mutex::new(0));

    let clone1 = global.clone();
    let thread1 = thread::spawn(move || {
        for _ in 0..COUNT {
            let mut value = clone1.lock().unwrap();
            *value += 1;
        }
    });

    let clone2 = global.clone();
    let thread2 = thread::spawn(move || {
        for _ in 0..COUNT {
            let mut value = clone2.lock().unwrap();
            *value -= 1;
        }
    });

    thread1.join().ok();
    thread2.join().ok();

    println!("final value: {:?}", global);
}

///
/// `RwLock`
///
/// RwLock就是“读写锁”。它跟Mutex很像，主要区别是对外暴露的API不一样。对Mutex内部的数据读写，
/// RwLock都是调用同样的lock方法；而对RwLock内部的数据读写，它分别提供了一个成员方法`read/write`来做这个事情。
///
/// - 同时允许多个读，最多只能有一个写；
/// - 读和写不能同时存在；
///
#[test]
fn _28_06_01_rwlock() {
    use std::sync::Arc;
    use std::sync::RwLock;
    use std::thread;

    const COUNT: u32 = 1_000_000;

    let global = Arc::new(RwLock::new(0));
    let clone1 = global.clone();
    let thread1 = thread::spawn(move || {
       for _ in 0..COUNT {
           let mut value = clone1.write().unwrap();
           *value += 1;
       }
    });

    let clone2 = global.clone();
    let thread2 = thread::spawn(move || {
       for _ in 0..COUNT {
           let mut value = clone2.write().unwrap();
           *value -= 1;
       }
    });

    thread1.join().ok();
    thread2.join().ok();

    println!("final value: {:?}", global);
}


///
/// Rust标准库提供了一些原子性操作的数据类型。它们在`std::sync::atomic`
///
/// 它们都是符合Sync的，可以在多线程之间共享.
///
#[test]
fn _28_07_01_atomic() {
    use std::sync::Arc;
    use std::sync::atomic::{AtomicIsize, Ordering};
    use std::thread;

    const COUNT: u32 = 1_000_000;

    // Atomic 系列类型同样提供了线程安全版本的内部可变性
    let global = Arc::new(AtomicIsize::new(0));

    let clone1 = global.clone();
    let thread1 = thread::spawn(move || {
        for _ in 0..COUNT {
            clone1.fetch_add(1, Ordering::SeqCst);
        }
    });

    let clone2 = global.clone();
    let thread2 = thread::spawn(move || {
        for _ in 0..COUNT {
            clone2.fetch_sub(1, Ordering::SeqCst);
        }
    });

    thread1.join().ok();
    thread2.join().ok();
    println!("final value: {:?}", global);
}

///
/// 与上一个版本相比，这段代码的区别在于：我们没有使用原子类型自己提供的`fetch_add` `fetch_sub`方法，
/// 而是使用了load把里面的值读取出来，然后执行加/减，操作完成后，再用store存储回去。编译程序我们看到，
/// 是可以编译通过的。再执行，出现了问题：这次的执行结果就不是保证为0了。
///
#[test]
fn _28_07_02_atomic() {
    use std::sync::Arc;
    use std::sync::atomic::{AtomicIsize, Ordering};
    use std::thread;

    const COUNT: u32 = 1000000;

    let global = Arc::new(AtomicIsize::new(0));
    let clone1 = global.clone();
    let thread1 = thread::spawn(move || {
        for _ in 0..COUNT {
            let mut value = clone1.load(Ordering::SeqCst);
            value += 1;
            clone1.store(value, Ordering::SeqCst);
        }
    });

    let clone2 = global.clone();
    let thread2 = thread::spawn(move || {
        for _ in 0..COUNT {
            let mut value = clone2.load(Ordering::SeqCst);
            value -= 1;
            clone2.store(value, Ordering::SeqCst);
        }
    });

    thread1.join().ok();
    thread2.join().ok();
    println!("final value: {:?}", global);
}


///
/// 死锁
///
///
#[test]
fn _28_08_01_deadlock() {
    use std::thread;
    use std::sync::{Mutex, Arc};
    use std::time::Duration;

    struct Philosopher {
        name: String,
        left: usize,
        right: usize,
    }

    impl Philosopher {
        fn new(name: &str, left: usize, right: usize) -> Philosopher {
            Philosopher {
                name: name.to_string(),
                left,
                right,
            }
        }

        fn eat(&self, table: &Table) {
            let _left = table.forks[self.left].lock().unwrap();
            println!("{} take left fork.", self.name);
            thread::sleep(Duration::from_secs(2));

            let _right = table.forks[self.right].lock().unwrap();
            println!("{} take right fork.", self.name);
            thread::sleep(Duration::from_secs(1));

            println!("{} is done eating.", self.name);
        }
    }

    struct Table {
        forks: Vec<Mutex<()>>,
    }

    let table = Arc::new(Table { forks:vec![
        Mutex::new(()),
        Mutex::new(()),
        Mutex::new(()),
        Mutex::new(()),
        Mutex::new(()),
    ]});

    let philosophers = vec![
        Philosopher::new("Judith Butler", 0, 1),
        Philosopher::new("Gilles Deleuze", 1, 2),
        Philosopher::new("Karl Marx", 2, 3),
        Philosopher::new("Emma Goldman", 3, 4),
        Philosopher::new("Michel Foucault", 4, 0),
    ];

    let handles: Vec<_> = philosophers.into_iter().map(|p| {
        let table = table.clone();

        thread::spawn(move || {
            p.eat(&table);
        })
    }).collect();

    for h in handles {
        h.join().unwrap();
    }

    // 5位哲学家都拿起了左边的筷子，都等待右边的筷子，都不愿释放自己已经拿到的筷子。
    // 大家进入无限的等待之中，程序无法执行。造成“死锁”。
    // 死锁不属于内存安全问题，它无法在编译阶段由静态检查解决。它属于一种逻辑错误。
}


///
/// 除了“锁”之外，Rust标准库还提供了一些其他线程之间的通信方式，比如Barrier等。Barrier是这样的一个类型，
/// 它使用一个整数做初始化，可以使得多个线程在某个点上一起等待，然后再继续执行。
///
#[test]
fn _28_09_01_barrier() {
    use std::sync::{Arc, Barrier};
    use std::thread;

    let barrier = Arc::new(Barrier::new(10));
    let mut handlers = vec![];
    for _ in 0..10 {
        let c = barrier.clone();
        // The same messages will be printed together.
        // You will NOT see any interleaving.
        let t = thread::spawn(move || {
            println!("before wait");
            c.wait();       // 所有线程都在某个点等待，再一起释放
            println!("after wait");
        });
        handlers.push(t);
    }

    for h in handlers {
        h.join().ok();
    }

}

///
/// `Condvar`
///
///
#[test]
fn _28_10_01_condvar() {
    use std::sync::{Arc, Mutex, Condvar};
    use std::thread;
    use std::time::Duration;

    let pair = Arc::new((Mutex::new(false), Condvar::new()));
    let pair2 = pair.clone();

    thread::spawn(move || {
        thread::sleep(Duration::from_secs(1));
        let &(ref lock, ref cvar) = &*pair2;
        let mut started = lock.lock().unwrap();
        *started = true;
        cvar.notify_one();
        println!("child thread {}", *started);
    });


    // wait for the thread to start up
    let &(ref lock, ref cvar) = &*pair;
    let mut started = lock.lock().unwrap();
    println!("before wait {}", *started);
    while !*started {
        started = cvar.wait(started).unwrap();
    }
    println!("after wait {}", *started);
}


///
/// 线程局部（Thread Local）的意思是，声明的这个变量看起来是一个变量，但它实际上在每一个线程中分别有自己独立的存储地址，是不同的变量，互不干扰。
#[test]
fn _28_11_01_thread_local() {
    use std::cell::RefCell;
    use std::thread;

    thread_local!{
        static FOO: RefCell<u32> = RefCell::new(1)
    };

    FOO.with(|f| {
        println!("main thread value1 {:?}", *f.borrow());
        *f.borrow_mut() = 2;
        println!("main thread value2 {:?}", *f.borrow());
    });

    let t = thread::spawn(move || {
       FOO.with(|f| {
           println!("child thread value1 {:?}", *f.borrow());
           *f.borrow_mut() = 3;
           println!("child thread value2 {:?}", *f.borrow());
       });
    });
    t.join().ok();

    FOO.with(|f| {
        println!("main thread value3 {:?}", *f.borrow());
    });
}










