
pub fn _02_02_01_bool() {

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
pub fn _02_02_02_char() {
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
pub fn _02_02_03_integer() {

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
pub fn _02_02_04_integer_overflow() {
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
pub fn _02_02_05_float() {
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
pub fn _02_02_06_pointer() {
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

pub fn _02_02_07_type_transform() {
    let var1: i8 = 41;
    let var2: i16 = var1 as i16;    // Rust 希望可以显式标记类型转换，以防止隐藏的bug

    let i = 42;
    // 先转换为*const i32, 再转换为*mut i32
    let p = &i as *const i32 as *mut i32;
    println!("{:p}", p);
}


























