mod adt {
    #[allow(dead_code)]
    pub struct S<'a, T: 'a> { pub b: &'a T }
}


///
/// 变量必须先声明，后使用。
///
pub fn _02_01_variable_declaration() {
    let _variable: i32 = 100;
}

/// `let` 关键字，既是声明语句，也是 **模式解构(pattern destructure)**
pub fn _02_03_pattern_destructure() {
    // `mut x`被视作一个组合
    let mut _x = 5;
    _x = 10;

    // `tuple`的模式解构
    let (mut _a, mut _b) = (1, 2);

    use crate::chap02::adt::*;

    // `struct`的模式解构
    let s1 = S { b: &true };
    let s2 = S { b: &s1 };
    let S { b: r1 } = s1;
    let S { b: &S { b: r2 } } = s2;
    println!("{} {} {} {}", s1.b, s2.b.b, r1, r2);
    println!("{:p} {:p} {:p} {:p}", s1.b, s2.b.b, r1, r2)


    // 所有变量必须被合理初始化才能被使用，否则出现：“error: use of possibly uninitialized variable: `x`
    // let _k: i32;
    // println!("{}", x);
}

/// 类型没有“默认构造函数”，变量没有“默认值”。
pub fn _02_03_initialization(condition: bool) {
    // 声明 x，不必使用 mut 修饰
    let x: i32;
    if condition {
        // 初始化 x，不需要 x 是 mut 的，因为这是初始化，不是修改
        x = 1;
        println!("{}", x);
    }
    // 如果条件不满足，x没有被初始化
    // 但是没关系，只要这里不使用 x 就没事
}

/// 占位符表示忽略这个变量绑定，后面不再用到
pub fn _02_03_placeholder() {
    let _ = "hello";
    // println!("{}", _);
    let arr = ['a', 'b', 'c'];
    let mut result: &str = "";
    for (_i, n) in arr.iter().enumerate() {
        match n {
            'a' => result = "Yeah!",
            _ => (),
        };
    };
    println!("{:?}", result);
}