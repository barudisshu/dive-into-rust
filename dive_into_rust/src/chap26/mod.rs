//!
//!
//! 在Rust里面，协程（Coroutine）是编写高性能异步程序的关键设施，生成器（Generator）是协程的基础。
//!
//!

///
/// 生成器的语法和闭包很像，但与闭包的区别在于生成器有`yield`关键字
///
/// 当闭包中有`yield`关键时，它就不是闭包，而是一个生成器。
///
/// - `let g=||{...yield...};`这句话是初始化了一个局部变量，它是一个生成器，此时并不执行生成器内部的代码；
/// - 调用`g.resume()`方法，此时会调用生成器内部的代码；
/// - 执行到`yield curr;` 这条语时，`curr`变量的值为1，生成器的方法此时会退出，`g.resume()` 方法的返回值是
/// `GeneratorState::Yielded(1)`，在main函数中，程序会打印出1；
///
#[test]
fn _26_01_01_generator() {
    use std::ops::{Generator, GeneratorState};
    use std::pin::Pin;

    let mut generator = || {
        let mut curr: u64 = 1;
        let mut next: u64 = 1;
        loop {
            let new_next = curr.checked_add(next);
            if let Some(new_next) = new_next {
                curr = next;
                next = new_next;
                yield curr; // <-- 新的关键字
            } else {
                return;
            }
        }
    };

    loop {
        unsafe {
            // resume
            match Pin::new(&mut generator).resume() {
                GeneratorState::Yielded(v) => println!("{}", v),
                GeneratorState::Complete(_) => return,
            }
        }
    }

}

#[test]
fn _26_02_01_generator() {
    // 方案二
    struct Fibonacci {
        curr: u64,
        next: u64,
    }

    impl Iterator for Fibonacci {
        type Item = u64;

        fn next(&mut self) -> Option<u64> {
            // 判断是否会溢出
            let new_next = self.curr.checked_add(self.next);

            if let Some(new_next) = new_next {
                // 先更新内部状态,再返回
                self.curr = self.next;
                self.next = new_next;
                Some(self.curr)
            } else {
                // 加法溢出,停止迭代
                None
            }
        }
    }

    fn fibonacci() -> Fibonacci {
        Fibonacci { curr: 1, next: 1 }
    }

    let mut it = fibonacci();

    while let Some(i) = it.next() {
        println!("{}", i);
    }
}