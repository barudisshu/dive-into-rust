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

    panic::catch_unwind(|| {
        let x: Option<i32> = None;
        x.unwrap();
        println!("interrupted. ");
    }).ok();

    println!("continue to execute. ");
}


























