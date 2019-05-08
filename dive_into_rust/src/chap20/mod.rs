//!
//! `Unsafe`
//!
//! 目前为止，Rust编译器的静态检查是不够用的，它没有办法自动推断出来这段代码究竟是不是安全的。
//! 需要使用unsafe关键字来保证代码安全性。
//!
//! Rust的unsafe关键字有以下几种用法：
//!
//! - 用于修饰函数fn;
//! - 用于修饰代码块；
//! - 用于修饰trait:
//! - 用于修饰impl;
//!
//! 当一个fn是unsafe的时候，意味着我们在调用这个函数的时候需要非常小心。它可能要求调用者满足一些其他的重要约束，
//! 而这些约束条件无法由编译器自动检查来保证。有unsafe修饰的函数，要么使用unsafe语句块调用，要么在unsafe函数中调用。
//!
//! unsafe调用是具有“传递性”的，unsafe函数的“调用者”也必须用unsafe修饰。
//!
//!
//!

use std::intrinsics::uninit;

///
/// 比如，`String::from_raw_parts`就是一个unsafe函数，它的签名如下：
///
/// ```
/// pub unsafe fn from_raw_parts(buf: *mut u8, length: usize, capacity: usize) -> String
/// ```
///
/// 之所以是unsafe的，是因为String类型对所有者有一个保证：它内部存储的是合法的utf-8字符串。而这个函数
/// 没有检查传递进来的这个缓冲区是否满足这个条件，所以使用者必须这样调用：
///
/// unsafe代码块，与普通代码比起来，多了一下几项能力：
///
/// - 对裸指针执行解引用操作；
/// - 读写可变静态变量；
/// - 读union或者写union的非Copy成员；
/// - 调用unsafe函数
///
/// 当unsafe修饰一个trait的时候，那么意味着实现这个trait也需要使用unsafe，因为编译器是没有能力推理验证这个impl是否正确的。
///
#[test]
fn _20_01_01_unsafe() {
    let ptr: u8 = 0;
    let len = 0;
    let capacity = 1;
    // 自己保证这个缓冲区包含的是合法的utf-8字符串
    let s = unsafe { String::from_raw_parts(ptr as *mut _, len, capacity) };
}


///
/// 裸指针
///
/// Rust提供了两种裸指针供我们使用，`*const T`和`*mut T`。我们可以通过`*mut T`修改所指向的数据，而`*const T`
/// 不能。在unsafe代码块中它们俩可以互相转换。
///
/// 裸指针相对于其他的指针，如Box、&、&mut来说，有以下区别：
///
/// - 裸指针可以为空，而编译器不保证裸指针一定指向一个合法的内存地址；
/// - 不会执行任何自动化清理工作，比如自动释放内存等；
/// - 裸指针赋值操作执行的是简单的内存浅复制，并且不存在borrow checker的限制。
///
#[test]
fn _20_01_02_unsafe() {

    // 创建裸指针是完全安全的行为，只有对裸指针执行“解引用”才是不安全行为，必须在unsafe语句块中完成

    let x = 1_u32;
    let mut y: u32 = 1;

    let raw_mut = &mut y as *mut u32 as *mut i32 as *mut i64;   // 这是安全的

    unsafe {
        *raw_mut = -1;      // 这是不安全的，必须在unsafe块中才能通过编译
    }

    println!("{:X} {:X}", x, y);
}

///
///
/// 在Rust中，&型引用、&mut型引用、Box指针，全部要求是合法的非空指针
///
#[test]
fn _20_01_03_unsafe() {

    fn raw_to_ref<'a>(p: *const i32) -> &'a i32 {
        unsafe {
            &*p     // 把一个裸指针，转换为一个共享引用
        }
    }
    let p : &i32 = raw_to_ref(std::ptr::null::<i32>());
    println!("{}", p);

}

///
/// unsafe中，允许写不规范的代码，因为编译器不会帮我们自动检查，
///
/// 比如同时存在多个&mut型指针，普通代码中会做borrowchk，但在unsafe中怎么写都可以，
///
/// 因此，大家不要滥用unsafe，能不写unsafe的情况，尽量不写
///
#[test]
fn _20_01_04_unsafe() {
    fn raw_to_ref<'a>(p: *const i32) -> Option<&'a i32> {
        if p.is_null() {
            None
        } else {
            unsafe { Some(&*p) }
        }
    }
    let p: Option<&i32> = raw_to_ref(std::ptr::null::<i32>());
    println!("{:?}", p);
}

///
/// 在标准库中，有一个`std::intrinsics`模块，它里面包含了一系列的编译器内置函数。
/// 这些函数都有一个`extern"rust-intrinsic"`修饰，它们看起来都像一种特殊的FFI外部函数，
/// 大家打开标准库的源代码`src/core/intrinsics.rs`，可以看到这些函数根本没有函数体，因为它们的实现是在编译器内部，
/// 而不是在标准库内部。调用它们的时候都必须使用unsafe才可以。编译器见到这些函数，就知道应该生成什么样的代码，
/// 而不是像普通函数调用一样处理。另外，intrinsics是藏在一个`feature gate`后面的，这个feature可能永远不会稳定，
/// 这些函数就不是准备直接提供给用户使用的。一般标准库会在这些函数基础上做一个更合适的封装给用户使用。
///
#[test]
fn _20_01_05_intrinsics() {

    // `fn transmute<T，U>（e：T）->U` 函数可执行强制类型转换
    let x = vec![1, 2, 3, 4, 5, 6];

    unsafe {
        // transmute和transmute_copy在`std::mem`模块中重新导出。用户如果需要，请使用这个模块，而不是`std::intrinsics`模块。
        let t: (usize, usize, usize) = std::mem::transmute_copy(&x);
        // 一个是指向堆上的指针，
        // 一个是指向内存空间的总大小
        // 一个是实际使用了的元素个数
        println!("{} {} {}", t.0, t.1, t.2);
    }
}

///
/// 内存读写
///
/// intrinsics模块里面有几个与内存读写相关的函数，
/// 比如`copy`、`copy_nonoverlapping`、`write_bytes`、`move_val_init`、`volatile_load`等。
/// 这些函数又在`std::ptr/std::mem`模块中做了个简单封装，然后暴露出来给用户使用。下面挑其中几个重要的函数介绍。
///
#[test]
fn _20_01_06_intrinsics() {

    fn swap<T>(x: &mut T, y: &mut T) {
        unsafe {
            let mut t: T = std::mem::MaybeUninit::<T>::uninit().assume_init();
            std::ptr::copy_nonoverlapping(&*x, &mut t, 1);
            std::ptr::copy_nonoverlapping(&*y, x, 1);
            std::ptr::copy_nonoverlapping(&t, y, 1);

            std::mem::forget(t);
        }
    }
}


///
/// 分割借用
///
/// 而对于数组切片，编译器的推理结果是将`x[_]`视为一个整体，`&x[A]`、`&x[B]`、`&x[C]`之间都算重叠。
/// 虽然读者可以看出来，`&mut x[0..2]`和`&mut x[3..4]`根本就是指向两块独立的内存区域，它们同时存在是完全安全的。
/// 但是编译器却觉得，`&mut x[A]`和`&mut x[B]`一定不能同时存在，否则就违反了alias+mutation的设计原则。
///
#[test]
fn _20_01_07_split_borrow() {
    let mut x = [1_i32, 2, 3];
    {
        let (first, rest)  : (&mut [i32], &mut [i32]) = x.split_at_mut(1);
        let (second, third): (&mut [i32], &mut [i32]) = rest.split_at_mut(1);
        first[0]  += 2;
        second[0] += 4;
        third[0]  += 8;
        println!("{:?} {:?} {:?}", first, second, third);
    }
    println!("{:?}", &x);
}

///
///
/// 协变
/// Rust不支持普通泛型参数类型的协变和逆变，只对声明周期泛型参数存在协变和逆变
///
#[test]
fn _20_05_01_covariant() {
    type StrRef<'a> = &'a str;

    fn print_str<'b>(s: StrRef<'b>) {
        println!("{}", s);
    }
    let s : StrRef<'static> = "hello";
    print_str(s);

    // `print_str`接受的参数类型是`Str-Ref<'b>`，而实际上传进来的参数类型是`StrRef<'static>`，
    // 这两个类型并不完全一致，因为`'b！='static`。但是Rust可以接受。
    // 这种现象在类型系统中被称为“协变”（covariance）和“逆变”（contravariance）。
}

///
/// 协变和逆变的定义如下。我们用`<"`符号记录子类型关系，对于泛型类型`C<T>`，
///
/// - 协变 若`T1<:T2`时满足`C<T1> <: C<T2>`，则`C`对于参数`T`是协变关系。
/// - 逆变 若`T1<:T2`时满足`C<T2> <: C<T1>`，则`C`对于参数`T`是逆变关系。
/// - 不变 上述两种都不成立。
///
/// 如果类型构造器保持了参数的子类型关系，就是协变；如果逆转了参数的子类型关系，就是逆变。其他情况，就是不变。
///
/// Rust不支持普通泛型参数类型的协变和逆变，只对生命周期泛型参数存在协变和逆变。
///
///
#[test]
fn _20_05_02_convariant() {

    fn test1<'a>(s: &'a &'static str) {
        let local: &'a &'a str = s;
    }

    // 出现生命周期错误。说明从`&'a mut &'static str`类型到`&'a mut &'a str`类型的转换是不安全的。
    // `&mut`型指针针对泛型`T`参数是不变的。
    // ILLEGAL: fn test2<'a>(s: &'a mut &'static str) {
    // ILLEGAL:     let local: &'a mut &'a str = s;
    // ILLEGAL: }

    // 编译通过，说明从`Box<&'static str>`类型到`Box<&'a str>`类型的转换是安全的。
    // `Box<T>`类型针对`T`参数是具备协变关系的。
    fn test3<'a>(s: Box<&'static str>) {
        let local: Box<&'a str> = s;
    }

    // 类型`fn(T) -> U`对于泛型参数T具备协变关系
    fn test_arg<'a>(f: fn(&'a str)) {
        let local: fn(&'static str) = f;
    }

    // 类型`fn(T) -> U`对于泛型参数U具备逆变关系
    fn test_ret<'a>(f: fn() -> &'a str) {
        f();
    }

    // 编译出现了生命周期不匹配的错误。说明`Cell<T>`类型针对`T`参数不具备协变关系。
    // 具备内部可变性的类型还有生命周期协变关系，可以构造出悬空指针的情况。
    // 所以需要编译器提供的UnsafeCell来表达针对类型参数具备“不变”关系的泛型类型
    // fn test5<'a>(s: std::cell::Cell<&'static str>) {
    //     let local: std::cell::Cell<&'a str> = s;
    // }

    // `*const T`针对`T`参数具备协变关系，而`*mut T`针对`T`参数是不变关系。
    // 比如标准库里面的`Box<T>`，它的内部包含了一个裸指针，这个裸指针就是用的`*const T`而不是`*mut T`。
    // 这是因为我们希望`Box<T>`针对`T`参数具备协变关系，而`*mut T`无法提供。
    // fn test6<'a>(s: *mut &'static str) {
    //     let local: *mut &'a str = s;
    // }
}


///
/// 在写unsafe代码的时候，我们经常会碰到一种情况，那就是一个类型是带有生命周期参数的，
/// 它表达的是一种借用关系。可以它内部是用裸指针实现的。请注意，裸指针是不带声明周期参数的。
/// 我们需要使用PhantomData来表达这个信息
///
///
#[test]
fn _20_05_03_phantom_data() {
    use std::fmt::Debug;
    use std::ptr::null;
    use std::marker::PhantomData;

    #[derive(Copy, Clone, Debug)]
    struct S;

    #[derive(Debug)]
    struct R<'a, T: Debug + 'a> {
        x: *const T,
        marker: PhantomData<&'a T>,
    }

    impl<'a, T:Debug> Drop for R<'a, T> {
        fn drop(&mut self) {
            unsafe {
                println!("Dropping R while S {:?}", *self.x);
            }
        }
    }

    impl<'a, T:Debug + 'a> R<'a, T> {
        pub fn ref_to<'b: 'a>(&mut self, obj: &'b T) {
            self.x = obj;
        }
    }

    let mut r = R { x: null(), marker: PhantomData };
    let local = S {  };
    r.ref_to(&local);
    //       ^^^^^^ 编译器获取到生命周期错误信息
}


///
/// 未定义行为，简称UB，
///
/// Rust中的UB被限制在一个较小范围，只有unsafe代码有可能制造出UB，这也是在写unsafe代码需要注意的地方
///
/// 下面列举一些undefined behavior，
///
/// - 数据竞争
/// - 解引用空指针或者悬空指针
/// - 使用未对齐的指针读写内存而不是使用`read_unaligned`或者`write_unaligned`
/// - 读取未初始化内存
/// - 破坏指针别名规则
/// - 通过共享引用修改变量(除非数据是被UnsafeCell包裹的)
/// - 调用编译器内置函数制造UB
/// - 给内置类型赋予非法值
/// - 给引用或者Box赋值为空或者悬空指针
/// - 给bool类型赋值为0和1之外的数字
/// - 给enum类型赋予类型定义之外的tag标记
/// - 给char类型赋予超过`char::MAX`的值
/// - 给str类型赋予非utf-8编码的值
///
///
/// Rust的unsafe最大的问题在于，到目前为止，依然没有一份官方文档来明确哪些东西是用户可以依赖的、哪些是编译器实现相关的、
/// 哪些是以后永远不变的、哪些是将来可能会有变化的。所以，哪怕用户能确保自己写出来的unsafe代码在目前版本上是完全正确的，
/// 也没办法确保不会在以后的版本中出问题。如果以后编译器的实现发生了变化，导致了unsafe代码无法正常工作，究竟算是编译器的bug
/// 还是用户错误地依赖了某些特性，还说不清楚。正式的unsafe guideline还在继续编写过程中。（当然这种错误情况几率是很低的，
/// 绝大多数用户使用unsafe的时候都是在FFI场景下，不会涉及那些精微细密的语义规则。）
///
///我们既不能过于滥用unsafe，也不该对它心怀恐惧。它只是表明，某些代码的安全性依赖于某些条件，
/// 而我们无法清晰地在代码中表达这些约束条件，因此无法由编译器帮我们自动检查。
///
///unsafe是Rust的一块重要拼图，充分理解unsafe的意义和作用，才能让我们更好地理解safe的来源和可贵。
///
#[test]
fn _20_06_01_undefined_behavior() {

}
