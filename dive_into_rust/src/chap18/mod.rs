//!
//! 泄露
//!
//! 如果引用计数智能指针出现了循环引用，就会导致内存泄露。
//!
//! 而Rust中也一样存在引用计数智能指针Rc，出现内存泄露
//!
//!


///
/// 内存泄露
///
#[test]
fn _18_01_01_mem_overflow() {

    use std::rc::Rc;
    use std::cell::RefCell;

    struct Node {
        next: Option<Rc<RefCell<Node>>>
    }
    impl Node {
        fn new() -> Node {
            Node { next: None }
        }
    }
    impl Drop for Node {
        fn drop(&mut self) {
            println!("drop");
        }
    }

    fn alloc_objects() {
        let node1 = Rc::new(RefCell::new(Node::new()));
        let node2 = Rc::new(RefCell::new(Node::new()));
        let node3 = Rc::new(RefCell::new(Node::new()));

        node1.borrow_mut().next = Some(node2.clone());
        node2.borrow_mut().next = Some(node3.clone());
        node3.borrow_mut().next = Some(node1.clone());
    }

    alloc_objects();
    println!("program finished.");

    {
        // 析构函数没有被调用，说明内存泄露已经发生，
        // Rust无法从根本上避免内存泄露
        // 通过循环引用构造内存泄露，需要同时满足三个条件：
        // 1) 使用引用计数指针；
        // 2) 存在内部可变性；
        // 3) 指针所指向的内容本身不是`'static`的
    }
}

///
/// 内存泄露属于内存安全
///
/// To put it another way, Rust gives you a lot of safety guarantees, but it doesn't protect
/// you from memory leaks (or dealocks, which turns out to be a very similar problem)
///
/// 内存泄露不是在语言层面能彻底解决的问题。
///
#[test]
fn _18_01_02_mem_safe() {

}


///
/// 析构函数泄露
///
/// 析构函数泄露是比内存泄露更严重的情况。因为析构函数是可以“自定义”的，析构函数里面
/// 可能调用了“任意的”代码
///
#[test]
fn _18_01_03_mem_drop() {
    // 以下示例目前无法编译通过，scoped已经被移除
    use std::thread;

    let mut vec = vec![0, 1, 2, 3, 4, 5, 6, 7];
    {
        let mut guards = Vec::new();
        for x in &mut vec {
            let guard = thread::scoped(move || {
                *x += 1;
            });
            guards.push(guard);
        }
        // guards析构，在析构函数中等待子线程被销毁
    }
    // 子线程已经全部退出
    println!("{:?}", vec);
}

