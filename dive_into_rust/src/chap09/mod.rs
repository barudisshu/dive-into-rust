///
///
/// “宏”是实现“元编程”的一种方式
/// 1. 编译期检查
/// 2. 编译期计算
/// 3. 自动代码生成
/// 4. 实现语法扩展
///
#[test]
fn _09_01_01_macro() {

    println!("number1 {} number2 {}", 1, 2); // 编译期检查、计算
    let v = vec![1, 2, 3, 4];   // 语法扩展
}

///
/// 自定义宏有两种方式：
/// 1. 通过标准库提供的`macro_rule!`宏实现
/// 2. 通过提供编译器扩展来实现
///
/// 编译器扩展只能在beta版本实现，它的API正在重新设计，还没正式定稿。
/// `macro_rule!`是标准库提供的提供编写简单宏的工具，它本身也是用编译器扩展来实现的。
/// 它可以提供一种“示范型(by example)”宏的编写方式。
///
///
/// 语法定义的标识符以`$`开头，类型支持`item`,`block`,`stmt`,`pat`,`expr`,`ty`,`itent`,`path`,`tt`
///
/// `+`代表一个或多个重复，`*`代表零个或多个重复
///
#[test]
fn _09_01_02_macro() {

    #[macro_export]
    macro_rules! hashmap {
        ($($key: expr => $val: expr), *) =>
        {{
        let mut map = ::std::collections::HashMap::new();
        $(map.insert($key, $val); )*;
        map
        }}
    }

    let counts = hashmap!['A' => 0, 'C' => 0, 'G' => 0, 'T' => 0];

    // `hashmap!`宏展开后等效于
    let mut map = ::std::collections::HashMap::new();
    map.insert('A', 0);
    map.insert('C', 0);
    map.insert('G', 0);
    map.insert('T', 0);

    assert_eq!(map, counts);
}

/// 过程宏(procedural macro)
///
/// 示例宏(by example)可以做一些简单逻辑，更复杂的逻辑需要用到过程宏来实现。
/// 前面说过，可以用`derive`帮我们自动`impl`某些`trait`。`derive`是一种编译器扩展，可以帮我们实现宏功能
/// Rust目前推出了macro 1.1版本，它还不是一个稳定的版本，最终版本会被macro 2.0代替。
///
#[test]
fn _09_01_03_macro() {

    use dive_into_rust_derive::AnswerFn;

    #[derive(AnswerFn)]
    struct Struct;

    assert_eq!(42, answer());
}