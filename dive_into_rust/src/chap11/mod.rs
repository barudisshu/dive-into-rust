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
fn _11_02_01_move() {

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
/// 默认的move语义是Rust的一个重要设计，但任何时候需要复制都去调用clone函数会显得非常繁琐，
/// 对于一些简单类型，在赋值是采用copy语义将会显得更简单
///
/// 普通变量绑定、函数传参、模式匹配场景下，凡是实现了`std::marker::Copy` trait的类型，都会执行copy语义
///
///
#[test]
fn _11_02_02_copy() {

    // 基本类型，比如数字、bool、字符等，都实现了`Copy` trait，因此具备copy语义
    let v1: isize = 0;
    let v2 = v1;
    assert_eq!(v1, v2);
}

///
/// 自定义类型，默认是没有 impl `Copy` trait的，
///
/// 实现的方法有两种：impl和编译器扩展
#[test]
fn _11_02_03_copy_impl() {

    struct Foo {
        data: i32
    }

    impl Copy for Foo {}

    // 基本上，要实现`Copy`语义，Copy和Clone的impl是同时出现的
    impl Clone for Foo {
        fn clone(&self) -> Self {
            Foo { data: self.data }
        }
    }

    let v1 = Foo { data: 0 };
    let v2 = v1;
    assert_eq!(v1.data, v2.data);
}

///
/// 采用编译器扩展derive attribute让编译器自动帮我们实现
#[test]
fn _11_02_04_copy_derive() {

    #[derive(Copy,Clone)]
    struct Foo {
        data: i32
    }

    let v1 = Foo { data: 0 };
    let v2 = v1;
    assert_eq!(v1.data, v2.data);
}


///
/// `Box`类型是Rust中一种常用的指针类型。它代表“拥有所有权的指针”
/// `Box`类型永远是move语义，不能是copy语义。原因很简单，Rust的copy语义是浅复制，对于`Box`这种类型而言，浅复制必定导致二次释放。
#[test]
fn _11_03_01_box() {

    struct T {
        value: i32
    }

    let p = Box::new(T{ value: 1});
    assert_eq!(1, p.value);
}


///
/// 并不是所有的类型都可以实现Copy trait。Rust规定，对于自定义类型，只有所有成员都实现了Copy trait，这个类型才有资格实现Copy trait。
/// 常见的数字类型、bool类型、共享借用指针&，都是具有Copy属性的类型。而Box、Vec、可写借用指针&mut等类型都是不具备Copy属性的类型。
///
/// - 对于数组类型，如果它内部的元素类型是Copy，那么这个数组也是Copy类型。
/// - 对于元组tuple类型，如果它的每一个元素都是Copy类型，那么这个tuple也是Copy类型。
/// - struct和enum类型不会自动实现Copy trait。只有当struct和enum内部的每个元素都是Copy类型时，编译器才允许我们针对此类型实现Copy trait。
#[test]
fn _11_04_01_copy_vs_clone() {
	// 我们可以认为，Rust中只有POD(C++语言中的Plain Old Data)类型才能有资格实现`Copy` trait.
	// 在Rust中，如果一个类型只包含POD数据类型的成员，并且没有自定义析构函数，那它就是POD类型。
	// 比如：整数、浮点数、只包含POD类型的数组等，都属于POD类型；
	// 而Box String Vec等不能按字节复制的类型，都不属于POD类型。
	// 但也并不是所有满足POD的类型都应该实现`Copy` trait，是否实现`Copy`取决于业务需求
}


///
/// `std::clone::Clone`有两个关联方法，`clone_from`和`clone`，`clone_from`是默认实现，依赖于`clone`方法的实现。
/// `clone`方法需要手动实现。
/// `clone`方法一般用于“基于语义的复制”操作。跟具体类型息息相关。
/// 对于`Box`类型、`clone`执行的是“深复制”；
/// 对于`Rc`类型，`clone`做的是把引用计数值加1.
#[test]
fn _11_04_02_clone() {
}


///
/// 自动derive可以减少重复的代码工作。derive会让编译器帮我们自动生成`impl Copy`和`impl Clone`的代码
/// 通过derive`方式自动实现`Copy`和手动实现`Copy`有微小区别。当类型具有泛型参数的时候，比如`struct MyStruct <T> {}`，通过
/// derive自动生成的代码会自动添加一个`T: Copy`的约束。
/// 目前，只有一部分固定的特殊trait可以通过derive来自动实现。
#[test]
fn _11_04_03_derive() {
}


/// `Copy`和`Clone`两者的区别在于：
/// - `Copy`内部没有方法，`Clone`内部有两个方法；
/// - `Copy` trait是给编译器用的，告诉编译器这个类型默认采用copy语义，而不是move语义。`Clone` trait是给程序员用的，我们必须手动调用clone方法；
/// - `Copy` trait不是想实现就能实现的，它对类型是有要求的，有些类型不可能impl Copy。`Clone` trait没有什么前提条件，任何类型都可以实现(`unsized`类型除外，因为无法使用`unsized`类型作为返回值)；
/// - `Copy` trait规定了这个类型在执行变量绑定、函数参数传递、函数返回等场景下的操作方式。即这个类型在这种场景下，必然执行的是“简单内存复制”，它由编译器控制。`Clone` trait里面的clone方法则由程序员自己控制；
/// - 如果不想自定义`Clone` trait`的操作，可以使用编译器扩展，在类型上加上`#[derive (Clone)]`，让编译器帮我们自动添加；
#[test]
fn _11_04_04_diff() {

}
