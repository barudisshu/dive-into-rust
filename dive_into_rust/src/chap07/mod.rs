
/// 模式解构
/// “Pattern Destructure”是Rust中一个重要且实用的设计。
///
/// - Rust的“模式解构”功能在语法上具有良好的一致性和扩展性；
/// - Rust的“模式解构”功能不仅出现在match语句中，还可以出现在`let`、`if-let`、`while-let`、函数调用、闭包调用等情景；
/// - Rust的“模式解构”功能可以应用于各种数据类型，包括但不限于`tuple`、`struct`、`enum`等，目前稳定版不支持`slice`模式匹配；
/// - Rust的“模式解构”功能要求“无遗漏”的分析(exhaustive case analysis)，确保不会因为不小心而漏掉某些情况；
/// - Rust的“模式解构”与Rust的核心所有权管理功能完全相容。
///
///
#[test]
fn _07_01_01_pattern_destructure() {

    let tuple = (1_i32, false, 3f32);

    // 典型地模式解构
    // let语句中，赋值号左边的内容就是“模式”，右边的内容就是需要被“解构”的内容
    let (head, center, tail) = tuple;

    // 模式解构的原则：
    // 构造和解构遵循类似的语法，我们怎么把一个数据解构组合起来的，我们就怎么把它拆解开来

    struct T1 (i32, char);

    struct T2 {
        item1: T1,
        item2: bool,
    }

    let x = T2 {
        item1: T1(0, 'A'),
        item2: false,
    };

    let T2 {
        item1: T1(value1, value2),
        item2: value3,
    } = x;

    println!("{} {} {}", value1, value2, value3);
}


/// 模式解构 match
#[test]
fn _07_01_02_pattern_destructure_match() {

    enum Direction {
        East, West, South, North
    }

    fn print(x: Direction) {
        match x {
            Direction::East => println!("East"),
            Direction::West => println!("West"),
            Direction::South => println!("South"),
            _ => println!("Other"),
        }
    }

    // Rust要求match的完整性，如果不想把每一种情况一一列举，可以用下划线表示“其它情况”，下划线必须放在最后，
    // 因为编译器运到有`_`的情况，后面的所有arm都不执行了！

    let x = Direction::East;
    print(x);
}

/// 下划线
#[test]
fn _07_01_03_pattern_destructure_underscore() {
    struct P(f32, f32, f32);
    // 参数类型是P，参数本身是一个模式，解构之后，变量x，y分别绑定了第一个和第三个成员
    fn calc(P(x, _, y): P) -> f32 {
        x * x + y * y
    }

    let t = P(1.0, 2.0, 3.0);
    assert_eq!(10.0, calc(t));

    // 下划线更像一个“关键字”，而不是普通的“标识符(identifier)”，把它当成普通标识符使用会有问题
    // ILLEGAL: let _ = 1_i32;
    // ILLEGAL: let x = _ + _;

    // 如果下划线后面跟上字母、数字或下划线，就成为一个正常的标识符。
    // `let _ = x;` 和 `let _y = x;`具有不一样的意义。
    // 如果变量`x`是非`Copy`类型，`let _ = x;`的意思是“忽略绑定”，此时会直接调用`x`的析构函数，
    // 我们不能在后面使用下划线`_`读取这个变量的内容；而`let _y = x;`的意思是“所有权转移”，`_y`是一个
    // 正常的变量名，`x`的所有权转移到了`_y`上，`_y`在后面可以继续使用。

    // “占位符”除了用`_`表示，还可以用`..`表示

    let x = (1, 2, 3);

    let (a, _, _) = x;

    // 其余全部省略
    let (k, ..) = x;

    // 省略所以元素
    let (m, .., n) = x;

    assert_eq!(1, a);
    assert_eq!(1, k);
    assert_eq!(1, m);
    assert_eq!(3, n);
}

/// match也是表达式
#[test]
fn _07_01_04_match_expression() {

    enum Direction {
        East, West, South, North
    }

    fn direction_to_int(x: Direction) -> i32 {
        match x {
            Direction::East => 10,
            Direction::West => 20,
            Direction::South => 30,
            Direction::North => 40,
        }
    }

    let x = Direction::East;
    let s = direction_to_int(x);
    assert_eq!(10, s);


    // 匹配值
    fn category(x: i32) -> &'static str {
        match x {
            -1 => "negative",
            0 => "zero",
            1 => "positive",
            _ => "error",
        }
    }

    assert_eq!("positive", category(1));

    // 多值匹配
    fn category_multi(x: i32) -> &'static str {
        match x {
            -1 | 1 => "true",
            0 => "false",
            _ => "error",
        }
    }

    assert_eq!("true", category_multi(1));

    // 范围匹配
    fn category_range(x: char) -> &'static str {
        match x {
            'a' ..= 'z' => "lowercase",
            'A' ..= 'Z' => "uppercase",
            _ => "something else",
        }
    }

    assert_eq!("uppercase", category_range('A'));
}

/// “匹配看守(match guards)”
#[test]
fn _07_01_05_guards() {

    enum OptionalInt {
        Value(i32),
        Missing,
    }

    let x = OptionalInt::Value(5);

    let k = match x {
        OptionalInt::Value(i) if i > 5 => "Got an int bigger than five!",
        OptionalInt::Value(..) => "Got an int!",
        OptionalInt::Missing => "No such luck.",
    };

    assert_eq!("Got an int!", k);

    // 无法覆盖所有情况时，需要手动加入一条ARM，避免编译错误，

    let a = 10;
    let o = match a {
        i if i > 5 => "bigger",
        i if i <= 5 => "small or equal",
        _ => unreachable!(),
    };

    assert_eq!("bigger", o);
}


/// 变量绑定
/// 变量绑定使用`@`符号，`@`符号前面是新声明的变量，后面是需要匹配的模式
///
#[test]
fn _07_01_06_variable_binding() {

    let x = 1;
    let b = match x {
        e @ 1 ..= 5 => true,    //`e`是新声明的变量，
        _ => false,
    };

    assert_eq!(true, b);

    // 当Pattern嵌套层次比较多，需要匹配更深层次作为条件
    fn deep_match(v: Option<Option<i32>>) -> Option<i32> {
        match v {
            // r 绑定到的是第一层`Option`内部，r的类型是`Option<i32>`
            // 与这种写法含义不一样：`Some(Some(r)) if (1..10).contains(r)`
            Some(r @ Some(1..10)) => r,
            _ => None,
        }
    }

    let x = Some(Some(5));
    assert_eq!(Some(5), deep_match(x));

    // 在使用`@`的同时使用`|`，需要保证在每个条件上都绑定这个名字
    let y = 5;
    let p = match y {
        e @ 1 .. 5 | e @ 8 .. 10 => e,
        _ => 0,
    };

    assert_eq!(0, p);
}

/// 引用绑定
#[test]
fn _07_01_07_reference_binding() {

    let x = 5_i32;
    match x {
        ref r => assert_eq!(5, *r),  // 此时r的类型是`&i32`
    };

    // 之所以在某些时候需要使用ref，是因为模式匹配的时候，有可能发生变量的所有权转移，
    // 使用ref就是为了避免出现所有权转移

    // ref关键字和引用符号&的区别在于：
    // ref是“模式”的一部分，它只能出现在赋值号左边，
    // &符号是借用运算符，是表达式的一部分，它只能出现在赋值号右边

    fn type_id(_: ()) {}

    let ref x = 5_i32;
    // 实际参数的类型肯定不是unit，此处必定有编译错误，通过编译错误，可以看到实参的具体类型
    // ILLEGAL: type_id(x);

    fn print_type_name<T>(_arg: &T) -> &'static str {
        unsafe {
            return std::intrinsics::type_name::<T>();
        }
    }

    let ref x = 5_i32;
    assert_eq!("&i32", print_type_name(&x));
}

/// &绑定
/// mut关键字和ref关键字一样，是“模式”的一部分
/// Rust中，所有的变量绑定默认都是“不可更改”的。
/// 只有使用了`mut`修饰的变量绑定才能修改数据
#[test]
fn _07_01_08_mut_binding() {

    let mut v = vec![1i32, 2, 3];
    v = vec![4i32, 5, 6];   // 重新绑定到新的Vec
    // ILLEGAL: v = vec![1.0f32, 2, 3];     // 类型不匹配，不能重新绑定

    // 重新绑定和“变量遮蔽”是完全不同的作用机制
    // “重新绑定”要求变量本身有mut修饰，并且不能改变这个变量的类型，
    // “变量遮蔽”要求必须重新声明一个新的变量，这个新变量与老变量之间的类型可以毫无关系

    let mut x: Option<String> = Some("hello".into());
    match &mut x {
        Some(i) => i.push_str(" world"),
        None => (),
    }

    assert_eq!("hello world", x.unwrap());
}

/// if-let和while-let模式匹配
#[test]
fn _07_01_09_if_let_while_let() {
    enum E<T> {
        A(T), B(T), C, D, E, F
    }

    let mut x = E::A(1);

    // if-let语句绑定
    let r = if let E::C | E::D = x { 1 } else { 2 };

    assert_eq!(2, r);

    // 等同于
    let r = match x {
        E::C | E::D => 1,
        _ => 2,
    };

    // 还可以有变量绑定
    while let E::A(k) | E::B(k) = x { assert_eq!(1, k); x = E::F; }     // while-let必须要有结束

    // 等同于
    match x {
        E::A(k) | E::B(k) => k,
        _ => 0,
    };
}

/// 函数和闭包参数做模式解构
#[test]
fn _07_01_10_closure_pattern() {
    // 函数可以接受一个结构体参数做模式解构的
    struct T {
        item1: char,
        item2: bool,
    }

    fn test(T{item1: arg1, item2: arg2}: T) -> (char, bool) {
        (arg1, arg2)
    }

    let x = T {
        item1: 'A',
        item2: false,
    };

    assert_eq!(('A', false), test(x));
}

