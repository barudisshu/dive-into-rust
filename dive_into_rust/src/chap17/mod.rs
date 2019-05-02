//!
//!
//! “解引用”(Deref)是“取引用”(Ref)的反操作。取引用，我们有`&`、`&mut`等操作符，对应的，解引用，我们有`*`操作符
//!
//!
//!



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