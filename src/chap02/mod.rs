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

/// 变量遮蔽
pub fn _02_01_01_variable_shadowing() {
    let x = "hello";
    println!("x is {}", x);

    let x = 5;
    println!("x is {}", x);

    // 变量遮蔽，通过let关键字声明，如果没有`let`关键字，就是对x的重新绑定

    // `v`必须是mut修饰，因为我们需要对它写入数据
    let mut v = Vec::new();
    v.push(1);
    v.push(2);
    v.push(3);

    // 让`v`成为只读变量，可读写变量v已经被遮蔽，无法再访问
    for i in &v {
        println!("{}", i);
    }

    // 反过来也可以让不可变变量，通过遮蔽方式创建一个新的、可变的同名变量
    let v = Vec::new();
    let mut v = v;
    v.push(1);
    println!("{:?}", v);
}


/// 类型推导
pub fn _02_01_02_type_infer() {
    // 没有明确标出变量的类型，但是通过字面量的后缀，
    // 编译器知道elem的类型为u8
    let elem = 5u8;

    // 创建一个动态数组，数组内包含的是什么元素类型可以不写
    let mut vec = Vec::new();
    vec.push(elem);
    // 到后面调用了push函数，通过elem变量的类型，
    // 编译器可以推导出vec的实际类型是Vec<u8>
    println!("{:?}", vec);

    let player_scores = [
        ("Jack", 20), ("Jane", 23), ("Jill", 18), ("John", 19),
    ];

    // players是动态数组，内部成员的类型没有指定，交给编译器自动推导
    let players: Vec<_> = player_scores
        .iter()
        .map(|&(player, _score) | {
            player
        }).collect();

    println!("{:?}", players);

    // 自动类型推导和“动态类型系统”是两码事。Rust依然是静态类型，一个变量的类型必须在编译阶段确定，且无法更改。

    // Rust只允许“局部变量/全局变量”实现类型推导，而函数签名等场景下是不允许的，这是故意这样设计的。
    // 因为局部变量只有局部影响，全局变量必须当场初始化而函数签名具有全局性影响。函数签名如果使用自动
    // 类型推导，可能导致某个调用的地方使用方式发生变化，它的参数、返回值类型就发生了变化，进而导致远处
    // 另一个地方的编译错误

    // 例如，iterator adapter中定义了`sum`方法，它的定义如下

    //    fn sum<S>(self) -> S
    //        where
    //            S: Sum<Self::Item>,

    // 它的泛型返回类型`S`必须实现`Sum`trait。因此编译器要求你指定sum的具体类型

    let a: i32 = (0..5).sum();
    let b: i32 = [0, 1, 2, 3, 4].iter().sum();
    assert_eq!(a, b);

    // illegal: let _c = [0, 1, 2, 3, 4].iter().sum();
    let c = [0, 1, 2, 3, 4].iter().sum::<i32>();
    assert_eq!(a, c);
}

/// 类型别名，主要作用是为了简化代码
pub fn _02_01_03_type_alias() {
    type Age = u32;

    fn grow(age: Age, year: u32) -> Age {
        age + year
    }

    let x: Age = 20;

    println!("20 years later: {}", grow(x, 20));

    // 类型别名还可以用在泛型场景，比如：
    type Double<T> = (T, Vec<T>);   // 它实际上是个复合类型，里面实际上是个tuple
}

/// 静态变量
pub fn _02_01_04_static_variable() {
    static _GLOBAL: i32 = 0;
    // 与`let`语句一样，static语句同样也是一个模式匹配。与let不同的是，
    // static声明的变量的生命周期是整个程序，从启动到退出。
    // static变量的生命周期永远是`'static`，它占用的内存空间也不会再执行过程中回收。
    // 这也是Rust中唯一的声明全局变量的方法。

    // 全局变量的使用有许多限制。这些限制是为了防止程序员写出不安全的代码：

    // 全局变量必须在声明的时候马上初始化；
    // 全局变量的初始化必须是编译期可确定的常量，不能包括执行期才能确定的表达式、语句和函数调用；
    // 带有mut修饰的全局变量，在使用的时候必须使用unsafe关键字。


    // 局部变量声明，可以留待后面初始化，只要保证使用前已经初始化即可
    let x;
    let y = 1_i32;
    x = 2_i32;
    println!("{} {}", x, y);

    // 全局变量必须声明的时候初始化，因为全局变量可以写到函数外面，被任意一个函数使用
    static G1: i32 = 3;
    println!("{}", G1);

    // 可变全局变量无论读写都必须用unsafe修饰
    static mut G2: i32 = 4;
    unsafe {
        G2 = 5;
        println!("{}", G2);
    }

    // 全局变量的内存不是分配在当前函数栈上，函数退出的时候，并不会销毁全局变量占用的内存空间，程序退出才会回收

    // Rust禁止在声明static变量的时候调用普通函数，或者利用语句块调用其它非const代码
    // 这样是允许的
    static _ARRAY: [i32; 3] = [1, 2, 3];
    // 这样是不允许的
    // ILLEGAL: static vec: Vec<i32> = { let mut v = Vec::new(); v.push(1); v };

    use std::sync::atomic::AtomicBool;
    static _FLAG: AtomicBool = AtomicBool::new(true);

    // Rust不允许用户在main函数之前或之后执行自己的代码。所以，比较复杂的static变量的初始化一般需要使用
    // lazy 方式，在第一次使用的时候初始化。在Rust中，如果用户需要使用比较复杂的全局变量初始化
    // 推荐使用lazy_static库
}

/// 常量
pub fn _02_01_05_constant() {
    // const 声明的是常量，而不是变量，因此一定不允许使用mut关键字修饰这个变量绑定
    // 它与static变量的最大区别在于：编译器并不一定会给const常量分配内存空间，在编译过程中，
    // 它很可能会被内联优化。
    // 以const声明的一个常量，也不具备类型let语句的模式匹配功能
    const _GLOBAL: i32 = 0;
}