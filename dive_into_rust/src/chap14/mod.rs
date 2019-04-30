//!
//!
//! Rust的借用需要避免两类问题，
//!
//! - Use After Drop
//! - Use After Change by an Alias
//!
//! 对于当前尚未被借用的对象来说，允许有：
//!
//! 1. 可以被`immut &`数次，所有者和任何借用方仅能读，不能写。
//! 2. 仅可被`mut &`一次，有且仅能有这个借用方读取和修改。
//!


///
/// 编译错误示例
///
#[test]
fn _14_01_01_error_case() {
    let i = 0;
    let p1 = &i;                   // i被冻结
    let p2 = &i;                   // i被冻结，仍然可读
    assert_eq!(*p1, i);

    // 变量绑定`i`、`p1`、`p2`指向的是同一个变量，我们通过不同的path访问同一块内存`p`、`*p1`、`*p2`
    // 所以它们存在“共享”。而且它们都只有只读的权限
}

///
/// “共享不可变、可变不共享”
///
#[test]
fn _14_01_02_error_case() {
    let mut i = 0;
    let p1 = &i;                   // i被冻结，仍然可读，但不可写
    // ILLEGAL: i = 1;
    assert_eq!(0, i);

    // 变量绑定`i`和`p1`已经互为alias，它们之间存在“共享”，因此必须避免“可变”
}

///
/// 可变变量`i`被`&mut`借用了，所有权转移到了借用方，变量`i`被锁住了LOCKED(不能写，不能读)
///
#[test]
fn _14_01_04_error_case() {
    let mut i = 0;
    let p1 = &mut i;                // i被上锁
    *p1 = 1;
    // ILLEGAL: let x = i;          // i不能读
    // ILLEGAL: i = 1;              // i不能写
    // ILLEGAL: let p2 = &mut i;    // cannot borrow `i` as mutable more than once at a time

    // `&mut`型借用也经常被称为“独占指针”
    // `&`型借用也经常被称为“共享指针”
}

///
/// 内存不安全示例：枚举
///
#[test]
fn _14_02_01_unsafe_case() {
    use std::fmt::Debug;
    #[derive(Debug)]
    enum StringOrInt {
        Str(String),
        Int(i64),
    }
    use StringOrInt::{Str, Int};
    let mut x = Str("Hello world".to_string());
    if let Str(ref insides) = x {
        x = Int(1);
//        println!("inside is {}, x says: {:?}", insides, x);
    }
}

///
/// 内存不安全示例：迭代器
///
#[test]
fn _14_02_02_unsafe_case() {
    let mut arr = vec!["ABC", "DEF", "GHI"];
    for item in &arr {
        // ILLEGAL: arr.clear();    // 出现野指针
    }
}

///
/// 内存不安全示例：悬空指针
///
#[test]
fn _14_02_03_unsafe_case() {
    let mut arr: Vec<i32> = vec![1, 2, 3, 4, 5];
    let p: &i32 = &arr[0];          // 出现悬空指针
    for i in 1..100 {
        // ILLEGAL: arr.push(i);
    }
}
