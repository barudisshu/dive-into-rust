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
/// ```rs
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
fn _20_05_01_convariant() {
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























