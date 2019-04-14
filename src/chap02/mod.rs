mod adt {
    #[allow(dead_code)]
    pub struct S<'a, T: 'a> { pub b: &'a T }
}


///
/// 变量必须先声明，后使用。
///
#[test]
fn _02_01_variable_declaration() {
    let _variable: i32 = 100;
}

/// `let` 关键字，既是声明语句，也是 **模式解构(pattern destructure)**
#[test]
fn _02_03_pattern_destructure() {
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
#[test]
fn _02_03_initialization(condition: bool) {
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
#[test]
fn _02_03_placeholder() {
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
#[test]
fn _02_01_01_variable_shadowing() {
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
#[test]
fn _02_01_02_type_infer() {
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
#[test]
fn _02_01_03_type_alias() {
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
#[test]
fn _02_01_04_static_variable() {
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
#[test]
fn _02_01_05_constant() {
    // const 声明的是常量，而不是变量，因此一定不允许使用mut关键字修饰这个变量绑定
    // 它与static变量的最大区别在于：编译器并不一定会给const常量分配内存空间，在编译过程中，
    // 它很可能会被内联优化。
    // 以const声明的一个常量，也不具备类型let语句的模式匹配功能
    const _GLOBAL: i32 = 0;
}

/// bool
#[test]
fn _02_02_01_bool() {

    let x = true;
    let y: bool = !x;

    let z = x && y;
    println!("{}", z);

    let z = x || y;
    println!("{}", z);

    let z = x & y;
    println!("{}", z);

    let z = x | y;
    println!("{}", z);

    let z = x ^ y;
    println!("{}", z);

    fn logical_op(x: i32, y: i32) {
        let z: bool = x < y;
        println!("{}", z);
    }

    //    if a >= b {
    //
    //    } else {
    //
    //    }
}

/// 字符类型
#[test]
fn _02_02_02_char() {
    let love = '❤';         // 可以直接嵌入任何 unicode 字符
    let c1 = '\n';          // 换行符
    let c2 = '\x7f';        // 8 bit 字符变量
    let c3 = '\u{7FFF}';    // unicode 字符

    println!("{} {} {} {}", love, c1, c2, c3);

    // 因为char类型的设计目的是描述任意一个unicode字符，因此它占据的内存空间不是1个字节，而是4个字节

    // 对于ASCII字符其实只需要占用一个字节的空间，因此Rust提供了单字节字符字面量来表示ASCII字符。
    // 我们可以使用一个字母b在字符或者字符串前面，代表这个字面量存储在u8类型数组中，这样占用空间
    // 比char型数组要小一些。示例如下：

    let x: u8 = 1;
    let y: u8 = b'A';
    let s: &[u8;5] = b"hello";
    let r: &[u8;14] = br#"hello \n world"#;

    use std::str::from_utf8;
    println!("{} {} {:?} {:?}", x, y, from_utf8(s).unwrap(), from_utf8(r).unwrap());
}

/// 整数类型
/// 整数类型主要区别特征是：有符号/无符号，占据空间大小
///
/// 整数类型        有符号     无符号
/// 8  bits         i8          u8
/// 16 bits         i16         u16
/// 32 bits         i32         u32
/// 64 bits         i64         u64
/// 128 bits        i128        u128
/// Pointer size    isize       usize
///
///
#[test]
fn _02_02_03_integer() {

    // 数字类型的字面量表示可以有许多方式
    let _var1: i32 = 32;         // 十进制表示
    let _var2: i32 = 0xFF;       // 以0x开头代表十六进制表示
    let _var3: i32 = 0o55;       // 以0o开头代表八进制表示
    let _var4: i32 = 0b1001;     // 以0b开头代表二进制表示

    // 所有的数字字面量中，可以在任意地方添加任意的下划线，以方便阅读：
    let _var5 = 0x_1234_ABCD;    // 使用下划线分隔数字，不影响语义，但极大提升了阅读体验
    let _var6 = 123usize;        // var6变量是unsize类型
    let _var7 = 0x_ff_u8;        // var7变量是u8类型
    let _var8 = 32;              // 不写类型，默认为i32类型


    let x: i32 = 9;
    println!("9 power 3 = {}", x.pow(3));
    // 或者直接对字面量调用函数
    println!("9 power 3 = {}", 9_i32.pow(3));
}


/// 整数溢出
#[test]
fn _02_02_04_integer_overflow() {
    // 在C语言中，对于无符号类型，算术运算永远不会overflow，如果超过表示范围
    // 则自动舍弃高位数据。对于有符号类型，如果发生了overflow，标准规定这是undefined behavior,
    // 也就是说随便怎么处理都可以。

    // 未定义行为有利于编译器做一些更激进的性能优化，但是这样的规定有可能导致在程序员不知情的某些极端场景下
    // 产生诡异的bug

    // Rust中希望能尽量减少“未定义行为”。
    // 默认情况下，在debug模式下编译器会自动插入整数溢出检查，一旦发生溢出，则会引发panic；
    // 在release模式下，不检查整数溢出，采用自动舍弃高位的方式。

    fn arithmetic(m: i8, n: i8) {
        // 加法运算，有溢出风险
        println!("{}", m + n);
    }

    let m: i8 = 120;
    let n: i8 = 120;
//    arithmetic(m, n);

    // 可以带上`-O`选项表示一个编译优化版本，执行后没有错误，它使用了自动截断策略
    // rustc -O test.rs

    // 某些场景下，确实需要精细地自主控制整数溢出的行为，可以调用标准库中的checked_*、saturating_*和wrapping_*系列函数

    let i = 100_i8;
    println!("checked {:?}", i.checked_add(i));
    println!("saturating {:?}", i.saturating_add(i));
    println!("wrapping {:?}", i.wrapping_add(i));

    // `checked_*`系列函数返回的类型是`Option<_>`，当出现溢出的时候，返回值是None;
    // `saturating_*`系列函数返回类型是整数，如果溢出，则给出该类型可表示范围的“最大/最小”值；
    // `wrapping_*`系列函数则是直接抛弃已经溢出的最高位，将剩下的部分返回。

    // 在很多情况下，整数溢出应该被处理为截断，即丢弃最高位。为了方便用户，标准库还提供了一个
    // 叫作`std::num::Wrapping<T>`的类型。它重载了基本的运算符，可以被当成普通整数使用。
    // 凡是被它包裹起来的函数，任何时候出现溢出都是截断行为。

    use std::num::Wrapping;
    let big = Wrapping(std::u32::MAX);
    let sum = big + Wrapping(2_u32);
    println!("{}", sum.0);
    // 不论用什么编译选项，上述代码都不会触发panic，任何情况下执行结果都是一致的。

}

/// 浮点类型
#[test]
fn _02_02_05_float() {
    // Rust提供了基于IEEE 754-2008标准的浮点类型
    // 按占据空间大小区分，分别为f32和f64
    let _f1 = 123.0f64;     // type f64
    let _f2 = 0.1f64;       // type f64
    let _f3 = 0.1f32;       // type f32
    let _f4 = 12E+99_f64;   // type f64 科学计数法
    let _f5: f64 = 2.;      // type f64

    // 与整数类型相比，Rust的浮点数类型相对复杂得多。浮点数的麻烦之处在于：它不仅可以表达正常的数值，还可以表达不正常的数值

    // 在标准库中，有一个std::num::FpCategory枚举，表示了浮点数可能的状态：

    enum FpCategory {
        Nan,
        Infinite,
        Zero,
        Subnormal,
        Normal,
    }

    // Zero表示0值，
    // Normal表示正常状态的浮点数，
    // Subnormal表示的浮点数精度比Normal精度低一点，

    // 变量small初始化为一个非常小的浮点数
    let mut small = std::f32::EPSILON;
    // 不断循环，让small越来越趁近于0，直到最后等于0的状态
    while small > 0.0 {
        small = small / 2.0;
        println!("{} {:?}", small, small.classify());
    }

    // Infinite表示“无穷大”，
    // Nan表示“不是数字”(not a number)。

    let x = 1.0f32 / 0.0;   // 无穷大
    let y = 0.0f32 / 0.0;   // 不是数字
    println!("{} {}", x, y);

    // 因为NaN的存在，浮点数是不具备“全序关系”(total order)的。
    let inf = std::f32::INFINITY;
    println!("{} {} {}", inf * 0.0, 1.0 / inf, inf / inf);
    // 输出结果为 NaN 0 NaN

    let nan = std::f32::NAN;
    println!("{} {} {}", nan < nan, nan > nan, nan == nan);
    // 输出结果为 false false false

}

/// 指针类型
#[test]
fn _02_02_06_pointer() {
    // 无GC的编程语言，如C、C++以及Rust，对数据的组织操作有更多的自由度，具体表现为：

    // 同一个类型，某些时候可以指定它在栈上，某些时候可以指定它在堆上。内存分配方式可以取决于使用方式，与类型本身无关。
    // 既可以直接访问数据，也可以通过指针间接访问数据。可以针对任何一个对象取得指向它的指针。
    // 既可以在复合数据类型中直接嵌入别的类型的实体，也可以使用指针，间接指向别的类型。
    // 甚至可能在复合数据类型末尾嵌入不定长数据构造处不定长的复合数据类型。

    // Rust中不止一种指针类型
    // Box<T>       指向类型T的、具有所有权的指针，有权释放内存
    // &T           指向类型T的借用指针，通常称为reference，无权释放内存，无权写数据，可租借数次，租借方拥有所有权。
    // &mut T       指向类型T的mut型借用指针，也称为mut reference，无权释放内存，owner和borrower拥有所有权，有且仅能被租借一次，
    // *const T     指向类型T的只读裸指针，没有生命周期信息，无权写数据
    // *mut T       指向类型T的可读写裸指针，没有生命周期信息，有权写数据

    // 除此之外还有一种封装起来的类型

    // Rc<T>        指向类型T的引用计数指针，共享所有权，线程不安全
    // Arc<T>       指向类型T的原子型引用计数指针，共享所有权，线程安全
    // Cow<'a, T>   clone-on-write，写时复制指针。可能是借用指针，也可能是具有所有权的指针
}

// `as` 表达式允许的类型转换如下，
// Integer or Float type        Integer or Float type
// C-like enum                  Integer type
// bool or char                 Integer type
// u8                           char
// *T                           *V where V: Sized *
// *T where T: Sized            Numeric type
// Integer type                 *V where V: Sized
// &[T;n]                       *const T
// Function pointer             *V where V: Sized
// Function pointer             Integer

// 更复杂的类型转换，一般使用标准库的From Into等trait
/// 类型转换
#[test]
fn _02_02_07_type_transform() {
    let var1: i8 = 41;
    let var2: i16 = var1 as i16;    // Rust 希望可以显式标记类型转换，以防止隐藏的bug

    let i = 42;
    // 先转换为*const i32, 再转换为*mut i32
    let p = &i as *const i32 as *mut i32;
    println!("{:p}", p);


}

/// 元组类型
///
#[test]
fn _02_03_01_tuple() {
    let a = (1i32, false);          // 元组中包含两个元素，第一个是i32类型，第二个是bool类型
    let b = ("a", (1i32, 2i32));    // 元组中包含两个元素，第二个元素本身也是元组

    // 如果元组仅包含一个元素，应该在后面添加一个逗号，以区分括号表达式和元组
    let a = (0,);
    let b = (0);

    // 访问元组内部元素有两种方法，一种是“模式匹配(pattern destructuring)”，另外一种是“数字索引”
    let p = (1i32, 2i32);
    let (a, b) = p;

    let x = p.0;
    let y = p.1;

    println!("{} {} {} {}", a, b, x, y);

    // 一个元素都没有的元组，叫做unit，是Rust中最简单的类型之一，也是占空间最小的类型之一。
    // 空元组和空结构体`struct Foo;`一样，都是占用0内存空间

    println!("size of i8 {}", std::mem::size_of::<i8>());       // 1 byte
    println!("size of char {}", std::mem::size_of::<char>());   // 4 bytes
    println!("size of '()' {}", std::mem::size_of::<()>());     // 0 byte


}

/// 结构体
#[test]
fn _02_03_02_struct() {
    // 结构体和元组类似，但用下标+字段访问
    struct Point {
        x: i32,
        y: i32,
    }
    let p = Point { x: 0, y: 0};
    println!("Point is at {} {}", p.x, p.y);

    // 如果局部变量名字和成员变量名字恰好一致，那么可以省略掉重复的冒号初始化

    let x = 10;
    let y = 20;
    let o = Point {x, y};
    println!("Point is at {} {}", o.x, o.y);

    // 模式匹配对结构体也适用
    let q = Point { x: 0, y: 0 };
    let Point {x: px, y: py} = q;
    println!("Point is at {} {}", px, py);

    // 同样，变量名和字段名相同时，可以简写
    let Point { x, y } = q;
    println!("Point is at {} {}", x, y);

    // Rust设计了一个语法糖，允许用一种简化的语法赋值使用另外一个struct的部分成员

    struct Point3d {
        x: i32,
        y: i32,
        z: i32,
    }

    fn default() -> Point3d {
        Point3d { x: 0, y: 0, z: 0 }
    }

    // 可以使用default()函数初始化其它的元素
    // ...expr 这样的语法，只能放在初始化表达式中，所有成员的最后最多只能有一个
    let origin = Point3d { x: 5, ..default() };
    let point = Point3d { z: 1, x: 2, ..origin };

    // 和tuple类似，struct内部成员也可以是空
    struct Foo1;
    struct Foo2();
    struct Foo3{}
}

/// 元组-结构体
/// tuple-struct
#[test]
fn _02_03_03_tuple_struct() {
    struct Color(i32, i32, i32);
    struct Point(i32, i32, i32);

    // 区别于tuple和struct，tuple-structs这种结构不能为空
    // 另外tuple属于原生类型primitive type

    struct T1 {
        v: i32
    }
    struct T2(i32);

    let v1 = T1 { v: 1};
    let v2 = T2(1);
    let v3 = T2 { 0: 1};

    let i1 = v1.v;
    let i2 = v2.0;
    let i3 = v3.0;

    // tuple-struct 的使用场景不多见
}

/// 枚举
#[test]
fn _02_03_04_enum() {

    enum Number {
        Int(i32),
        Float(f32),
    }

    fn read_num(num: &Number) {
        match num {
            &Number::Int(value) => println!("Integer {}", value),
            &Number::Float(value) => println!("Float {}", value),
        }
    }

    let n: Number = Number::Int(10);
    read_num(&n);

    println!("Size of Number: {}", std::mem::size_of::<Number>());
    println!("Size of i32:    {}", std::mem::size_of::<i32>());
    println!("Size of f32:    {}", std::mem::size_of::<f32>());

    enum Foo {
        Bar,            // 0     , 若声明的第一个变数(variant)没有指定，则初始化为0
        Baz = 123,      // 123   , 对于每个未被指定的判别式，设置比前一个高的值
        Quux,           // 124   , enum的设值类型仅可以是isize
    }

    let baz_discriminant = Foo::Baz as u32;
    assert_eq!(baz_discriminant, 123);

    enum Animal {
        Dog = 1,
        Cat = 2 | 3 | 4,// 位运算，结果为7
        Tiger,
    }

    let x = Animal::Tiger as isize;
    assert_eq!(x, 8);

    let arr = [1, 2, 3, 4, 5];
    let v: Vec<Option<&i32>> = arr.iter().map(Some).collect();
    println!("{:?}", v);
}

/// 类型递归定义
#[test]
fn _02_03_05_type_recursion() {

    struct Recursive {
        data: i32,
        rec: Box<Recursive>,    // 通过指针间接引用的方式，控制递归类型的内存
    }
}
