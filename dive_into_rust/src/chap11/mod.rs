///
///
///
/// “所有权”表达的意义有：
/// - 每个值再Rust中都有一个变量来管理它，这个变量就是这个值、这块内存的所有者；
/// - 每个值再一个时间点只有一个管理者；
/// - 当变量所在的作用域结束的时候，变量以及它代表的值将会被销毁。
///
///
#[test]
fn _11_01_01_ownership() {

    // 声明了变量s，并用String类型初始化，变量s就是该字符串的“所有者”
    let mut s = String::from("hello");
    s.push_str(" world");
    assert_eq!("hello world", s);

    // 当s离开了它所在的scope，它会被释放回收
}

///
///
/// 一般把变量从出生到死亡的整个阶段，叫做一个变量的“生命周期”。
///
#[test]
fn _11_01_02_ownership() {

    // 每个值只有一个所有者。变量s的生命周期从声明开始，到move给s1就结束了。
    // 中间所有权的转移，并不会将这个字符串本身重新销毁再创建
    let s = String::from("hello");
    let s1 = s;
    //ILLEGAL: println!("{}", s);
}

/// Rust不能做“赋值运算符重载”，需要“深复制”。
/// 调用`clone`方法，需要对应类型impl标准库的`std::clone::Clone`
#[test]
fn _11_01_03_ownership() {
    let s = String::from("hello");
    let s1 = s.clone();
    assert_eq!(s, s1);
}

///
/// 移动语义
///
/// 一个变量可以把它拥有的值转移给另外一个变量，称为“所有权转移”。赋值语句、函数调用、函数返回等，都可能导致所有权转移。
///
/// 移动语义是所有类型的默认语义。
///
#[test]
fn _11_01_04_move() {

    fn create() -> String {
        let s = String::from("hello");
        return s;   // 所有权转移，从函数内部移动到外部
    }

    fn consume(s: String) {
        (s);        // 所有权转移，从函数外部移动到内部
    }

    let s = create();
    consume(s);
}

///
///
///
#[test]
fn _11_01_05_copy() {

}