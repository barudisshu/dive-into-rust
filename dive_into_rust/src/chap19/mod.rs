//!
//!
//! Panic
//!
//! Panic属于一种“fail fast”机制，处理那种万不得已的情况
//!
//! 在Rust中，Panic的实现机制有两种方式：`unwind`和`abort`
//!
//! - `unwind`方式在发生panic的时候，会一层层地退出函数调用栈，在此过程中，当前栈内的局部变量还可以正常析构。
//! - `abort`方式在发生panic的时候，会直接退出整个程序。
//!
//! 常见操作系统下，默认情况下，编译器使用的是unwind方式。但是unwind并不是在所有平台都获得良好支持。在某些嵌入式
//! 系统上，unwind根本无法实现，或者占用资源太多。这时可以选择使用abort方式
//!
//! 编译器提供了一个选项，供用户指定panic的实现方式，
//!
//! ```txt
//! rustc -C panic=unwind test.rs
//! rustc -C panic=abort test.rs
//! ```
//!
//! panic并不意味着“内存不安全”，恰恰相反，它是阻止“内存不安全”的利器。内存不安全造成的问题比程序突然退出要严重得多。
//!

///
///
/// Option为None时，尝试调用unwrap会触发panic
///
#[test]
fn _19_01_01_panic() {
    let x: Option<i32> = None;
    // PANIC: x.unwrap();
}

///
/// Rust提供了一些工具函数，让用户在代码中终止栈展开
///
/// panic出现的场景一般是：如果继续执行下去就会有极其严重的内存安全问题，这种时候让程序继续执行导致的危害
/// 比崩溃更严重，此时panic就是最后的一种错误处理机制。它的主要用处参考下面的情况：
///
/// - 在FFI场景下的时候，当C语言调用了Rust的函数，在Rust内部出现了panic，如果这个panic在Rust内部没有处理好，直接
/// 扔到了C代码中，会导致C语言产生“未定义行为”（undefined behavior）。
/// - 某些高级抽象机制需要阻止栈展开，比如线程池。如果一个线程池出现了panic，我们希望只把这个线程关闭，而不至于将
/// 整个线程池“拖下水”
///
#[test]
fn _19_01_02_panic() {
    use std::panic;

    // catch_unwind 不是'try-catch'的替换模式，它不能在程序中作正常的逻辑处理
    panic::catch_unwind(|| {
        let x: Option<i32> = None;
        x.unwrap();
        println!("interrupted. ");
    }).ok();

    println!("continue to execute. ");
}

///
/// C++中引入了“异常”这个机制之后，同时也带入了一个“异常安全”(exception safety)的概念。
/// 异常安全存在四种层次的保证：
///
/// - `No-throw` ——这种层次的安全性保证了所有的异常都在内部正确处理完毕，外部毫无影响；
/// - `Strong exception safety` ——强异常安全保证可以保证异常发生的时候，所有的状态都可以“回滚”到初始状态，不会导致状态不一致的问题；
/// - `Basic exception safety` ——基本异常安全保证可以保证异常发生的时候不会导致资源泄露；
/// - `No exception safety` ——没有任何异常安全保证；
///
/// 当我们在系统中使用了“异常”的时候，就一定要想清楚，每个组件应该提供那种层级的异常安全保证。在Rust中，
/// 这个问题同样存在，但是一般叫做panic safety，于“异常”说的是同一件事情。
///
#[test]
fn _19_01_03_panic_safety() {
    use std::panic;
    use std::panic::AssertUnwindSafe;

    let mut x: Vec<i32> = vec![1];
    let mut y: Vec<i32> = vec![2];

    // 使用AssertUnwindSafe这个类型，来确保catch_unwind函数约束
    panic::catch_unwind(AssertUnwindSafe(|| {
        x.push(10);
        panic!("user panic");
        y.push(100);
    })).ok();

    println!("Observe corrupteed data. {:?} {:?}", x, y);
}

///
/// 多线程中，在某个线程中制造一个panic
///
#[test]
fn _19_01_04_panic_thread() {

    // 在thread2中，在达到某个条件的情况下会发生panic。这个panic是在Mutex锁定的状态下发生的。
    // 这时，标准库会将Mutex设置为一个特殊的称为poisoned状态。处在这个状态下的Mutex，再次调用lock，
    // 会返回Err状态。它里面依然包含了原来的数据，只不过用户需要显式调用into_inner才能使用它。

    use std::sync::Arc;
    use std::sync::Mutex;
    use std::thread;

    const COUNT: u32 = 1_000_000;

    let global = Arc::new(Mutex::new(0));

    let clone1 = global.clone();
    let thread1 = thread::spawn(move || {
       for _ in 0..COUNT {
           match clone1.lock() {
               Ok(mut value) => * value +=1,
               Err(poisoned) => {
                   let mut value = poisoned.into_inner();
                   *value += 1;
               }
           }
       }
    });

    let clone2 = global.clone();
    let thread2 = thread::spawn(move || {
        for _ in 0..COUNT {
            let mut value = clone2.lock().unwrap();
            *value -= 1;
            if *value < 100_000 {
                println!("make a panic");
                panic!("");
            }
        }
    });

    thread1.join().ok();
    thread2.join().ok();
    println!("final value: {:?}", global);
}

