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
/// - 循环调用`g.resume()`方法，此时再次进入到生成器内部的代码中；
/// - 此时生成器会直接从上次退出的那个地方继续执行，跳转到loop循环的开头，计算`curr next new_next`这几个变量新的值，
/// 然后再到`yield curr;` 这条语句返回；
/// - 如此循环往复，一直到加法计算溢出，生成器调用了`return;`语句，此时`main`函数那边会匹配上`GeneratorState::Complete`
/// 这个分支，程序返回，执行完毕。
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

///
/// 任何生成器，总能找到某种办法改写为功能相同的迭代器。
/// 下面是将上面生成器，改写成迭代器的例子
///
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

    // 迭代器模式是一种典型地“拉”模式，它也经常被称为“惰性求值”(lazy evaluation)。生成器在这一点上与迭代器是一样的，
    // 也需要使用者调用方法把数据拉出来。它们一个用的是`next`方法，一个用的是`resume`方法，虽然方法的签名有所不同，但使用上差不多。
}


///
/// 相对于“立即求值”(eager evaluation)。惰性求值更灵活，还节省了一个临时的占用很大的内存空间的容器
///
#[test]
fn _26_03_01_eager_evaluation() {
    // 方案三
    fn collector() -> Vec<u64> {
        let mut res = vec![];
        let mut curr: u64 = 1;
        let mut next: u64 = 1;
        loop {
            let new_next = curr.checked_add(next);
            if let Some(new_next) = new_next {
                curr = next;
                next = new_next;
                res.push(curr);
            } else {
                break;
            }
        }
        return res;
    }

    let collected = collector();
    let mut it = collected.iter();
    while let Some(i) = it.next() {
        println!("{}", i);
    }
}


///
/// 生成器的原理：
///
/// 编译器把生成器自动转换成了一个匿名类型，然后对这个类型实现了`Generator`这个trait。
/// 这种处理手法和闭包非常相似。和闭包一样，生成器也可以捕获当前环境中的局部变量，并且可以用move做修饰，
/// 捕获的环境变量都是当前生成器的成员，捕获规则也与闭包一样。
///
/// ```rs
/// trait Generator {
///     type Yield;
///     type Return;
///     // 至少到目前为止，resume方法还不能接受额外参数，这个限制条件以后可能会放宽
///     // 目前resume方法还是不稳定版本，以后应该会去掉unsafe，self的类型也会有所变化
///     unsafe fn resume(&mut self) -> GeneratorState<Self::Yield, Self::Return>;
/// }
/// ```
/// 生成器内部的语句会成为成员函数`resume()`的方法体，
/// 源代码中的`yield`语句都被替换成了普通的`return`语句，且返回的是`Generator-State::Yielded(_)`；
/// 源代码中的`return`语句依然是`return`语句，但返回的是`GeneratorState::Complete(_)`。
///
/// 生成器的特点是，每次yield退出之后，当前的局部变量会保持当前的值不变，下一次被调用resume再次进来执行的时候，
/// 会继续从上次yield的那个地方继续执行，局部变量是无须再次初始化的。
///
/// 目前生成器并不是一个稳定功能，它还有一些问题没有解决。最主要的一个问题是如何使得借用跨yield存在。
///
///
#[test]
fn _26_04_01_principle() {

    //
    let _g = || {
        let local = 1;
        let ptr = &local;
        yield local;
        yield *ptr;
    };

    // 编译，出错：borrow may still be in use when generator yields
}


///
///
/// Rust设计这个生成器，主要目的在于，基于生成器设计一套协程（Coroutine）的方案，从而方便编写大规模高性能异步程序。
/// 所谓协程，指的是一种用户态的非抢占式的多任务机制。它也可以实现多任务并行。跟线程相比，它的最大特点是它不是被内核调度的，
/// 而是由任务自己进行协作式的调度。协程的实现方案一般可以分为`stackful`以及`stackless`两种。
/// Rust的协程采用的是`stackless coroutine`的设计思路。
///
/// 在Rust语言和标准库中，只引入了极少数的关键字、trait和类型。async和await关键字是目前许多语言都采用的主流方案，
/// 使用关键字而不是用宏来做API，有助于社区的统一性，避免不同的异步方案使用完全不一样的用户API。
/// 引入关键字使用的是edition方案，所以不会造成代码不兼容问题。标准库中只有极少数必须的类型，这也是Rust一贯的设计思路。
/// 但凡是可以在第三方库中实现的，一律在第三方库中实现，哪怕这个库本来就是官方核心组维护的，
/// 这样做可以让这个库的版本升级更灵活，有助于标准库的稳定性。
///
/// 目前协程的设计仍在争论当中，在2019年后请参考Rust的网络编程部分
///
#[test]
fn _26_05_01_coroutine() {

}






































