//!
//! 内部可变性
//!
//! Rust的borrow checker的核心思想是“共享不可变，可变不共享”
//!
//!
//!
//!

#[test]
fn _16_01_01_interior_mutability() {

    use std::rc::Rc;
    let r1 = Rc::new(1);
    println!("reference count {}", Rc::strong_count(&r1));
    let r2 = r1.clone();
    println!("reference count {}", Rc::strong_count(&r2));
}