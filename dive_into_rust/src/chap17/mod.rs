//!
//!
//! “解引用”(Deref)是“取引用”(Ref)的反操作。取引用，我们有`&`、`&mut`等操作符，对应的，解引用，我们有`*`操作符
//!
//!
//!



use core::borrow::Borrow;

///
///
///
///
#[test]
fn _17_01_01_deref() {

    let v1 = 1;
    let p = &v1;        // 取引用
    let v2 = *p;        // 解引用
    assert_eq!(v1, v2);
}

///
/// 自定义解引用
/// 解引用的操作可以被自定义。方法是，实现标准库中的`std::ops::Deref`或者`std::ops::DerefMut`这两个trait
///
///
/// ```rs
///pub trait Deref {
///  type Target: ?Sized;
///  fn deref(&self) -> &Self::Target;
///}
///
///pub trait DerefMut: Deref {
///  fn deref_mut(&mut self) -> &mut Self::Target;
///}
/// ```
///
///
#[test]
fn _17_01_02_deref() {
}


///
/// 自动解引用
#[test]
fn _17_01_02_auto_deref() {

    let s = "hello";
    println!("length: {}", s.len());
    println!("length: {}", (&s).len());
    println!("length: {}", (&&&&&&&&&&&&&&s).len());
}

///
/// 自动解引用的用处
///
#[test]
fn _17_01_03_auto_deref() {
    use std::rc::Rc;

    let s = Rc::new(String::from("hello"));
    // Rc类型本身并没有`bytes()`方法，所以编译器会尝试自动deref，试试`s.deref().bytes()`
    // String类型其实也没有`bytes()`方法，但是String可以继续deref，于是再试试`s.deref().deref().bytes()`
    println!("{:?}", s.bytes());

    // 实际上以下写法在编译器看起来是一样的
    println!("length: {}", s.len());
    println!("length: {}", s.deref().len());
    println!("length: {}", s.deref().deref().len());
    println!("length: {}", (*s).len());
    println!("length: {}", (&*s).len());
    println!("length: {}", (&**s).len());
}


///
/// 有时候需要手动处理
///
#[test]
fn _17_01_04_handle_deref() {
    use std::rc::Rc;
    use std::ops::Deref;
    fn type_of(_: ()) {}

    let s = Rc::new(Rc::new(String::from("hello")));
    let s1 = s.clone();         // (1)
    let ps1 = (*s).clone();
    let pps1 = (**s).clone();
}

///
/// 有时候需要手动处理
///
#[test]
fn _17_01_05_handle_deref() {
    use std::rc::Rc;
    use std::ops::Deref;
    fn type_of(_: ()) {}

    let s = String::new();
    // 1.
    match s.deref() {
        "" => {}
        _ => {}
    }

    // 2.
    match &*s {
        "" => {}
        _ => {}
    }

    // 3.
    match s.as_ref() {
        "" => {}
        _ => {}
    }

    // 4.
    match s.borrow() {
        "" => {}
        _ => {}
    }

    // 5.
    match &s[..] {
        "" => {}
        _ => {}
    }
}
///
///
/// 智能指针
///
#[test]
fn _17_02_01_smart_pointer() {

}