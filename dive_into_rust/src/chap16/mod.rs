//!
//! 内部可变性
//!
//! Rust的borrow checker的核心思想是“共享不可变，可变不共享”
//!
//! “内部可变性”的概念，是与“承袭可变性”（inherited mutability）相对应的。
//!
//! 可变还是不可变取决于变量的使用方式，这就叫作“承袭可变性”。
//!
//! 如果我们用`let var: T;` 声明，那么`var`是不可变的，同时`var`内部的所有成员都是不可变的；
//! 如果我们用`let mut var: T;`声明，那么`var`是可变的，相应的，它的内部所有成员也都是可变的。
//!
//! 我们不能在类型声明的时候指定可变性，比如在struct中对某部分成员使用mut修饰，这是不合法的。我们只能在变量
//! 声明的时候指定可变性。我么也不能针对变量的某一部分成员指定可变性，其他部分保持不变。
//!
//! 常见的具备内部可变性特点的类型有：
//! `Cell` `RefCell` `Mutex` `RwLock` `Atomic*`等。其中`Cell`和`RefCell`是只能用在单线程环境下的具备内部可变性的类型。
//!
//!
//!

///
/// `Cell`，
/// 如果我们有共享引用指向一个对象，那么这个对象就不会被更改了。因为在共享引用存在的期间，不能有可变引用同时
/// 指向它，因此它一定是不可变的。
#[test]
fn _16_01_01_interior_mutability() {


    use std::rc::Rc;
    let r1 = Rc::new(1);
    println!("reference count {}", Rc::strong_count(&r1));
    let r2 = r1.clone();
    println!("reference count {}", Rc::strong_count(&r2));
}

///
///
///
#[test]
fn _16_01_02_interior_mutability() {
    use std::cell::Cell;
    let data: Cell<i32> = Cell::new(100);
    let p = &data;
    data.set(10);
    println!("{}", p.get());

    p.set(20);
    println!("{:?}", data);
}