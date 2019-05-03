//!
//!
//! “解引用”(Deref)是“取引用”(Ref)的反操作。取引用，我们有`&`、`&mut`等操作符，对应的，解引用，我们有`*`操作符
//!
//!
//!



///
///
///
///
#[test]
fn _17_01_01_deref() {

    let v1 = 1;
    let p = &v1;        // 取引用
    let v2 = *p;        // 解引用
    assert_eq!(v1, v2);
}

///
/// 自定义解引用
/// 解引用的操作可以被自定义。方法是，实现标准库中的`std::ops::Deref`或者`std::ops::DerefMut`这两个trait
///
///
/// ```rs
///pub trait Deref {
///  type Target: ?Sized;
///  fn deref(&self) -> &Self::Target;
///}
///
///pub trait DerefMut: Deref {
///  fn deref_mut(&mut self) -> &mut Self::Target;
///}
/// ```
///
///
#[test]
fn _17_01_02_deref() {
}


///
/// 自动解引用
#[test]
fn _17_01_02_auto_deref() {

    let s = "hello";
    println!("length: {}", s.len());
    println!("length: {}", (&s).len());
    println!("length: {}", (&&&&&&&&&&&&&&s).len());
}

///
/// 自动解引用的用处
///
#[test]
fn _17_01_03_auto_deref() {
    use std::rc::Rc;
    use std::ops::Deref;

    let s = Rc::new(String::from("hello"));
    // Rc类型本身并没有`bytes()`方法，所以编译器会尝试自动deref，试试`s.deref().bytes()`
    // String类型其实也没有`bytes()`方法，但是String可以继续deref，于是再试试`s.deref().deref().bytes()`
    println!("{:?}", s.bytes());

    // 实际上以下写法在编译器看起来是一样的
    println!("length: {}", s.len());
    println!("length: {}", s.deref().len());
    println!("length: {}", s.deref().deref().len());
    println!("length: {}", (*s).len());
    println!("length: {}", (&*s).len());
    println!("length: {}", (&**s).len());
}


///
/// 有时候需要手动处理
///
#[test]
fn _17_01_04_handle_deref() {
    use std::rc::Rc;
    use std::ops::Deref;
    fn type_of(_: ()) {}

    let s = Rc::new(Rc::new(String::from("hello")));
    let s1 = s.clone();         // (1)
    let ps1 = (*s).clone();
    let pps1 = (**s).clone();
}

///
/// 有时候需要手动处理
///
#[test]
fn _17_01_05_handle_deref() {
    use std::rc::Rc;
    use std::ops::Deref;
    use std::borrow::Borrow;
    fn type_of(_: ()) {}

    let s = String::new();
    // 1.
    match s.deref() {
        "" => {}
        _ => {}
    }

    // 2.
    match &*s {
        "" => {}
        _ => {}
    }

    // 3.
    match s.as_ref() {
        "" => {}
        _ => {}
    }

    // 4.
    match s.borrow() {
        "" => {}
        _ => {}
    }

    // 5.
    match &s[..] {
        "" => {}
        _ => {}
    }
}
///
///
/// 智能指针
///
/// 目前为止，我们接触的示例中都是一块内存总是只有唯一的一个所有者。当这个变量绑定自身消亡的时候，
/// 这块内存就会被释放。引用计数智能指针给我们提供了另外一种选择：一块不可变内存可以有多个所有者，
/// 当所有的所有者消亡后，这块内存才会被释放。
///
/// Rust中提供的引用计数指针有`std::rc::Rc<T>`类型和`std::sync::Arc<T>`类型。
///
/// Rc类型和Arc类型的主要区别是：
/// - `Rc`类型的引用计数是普通整数操作，只能用在单线程中；
/// - `Arc`类型的引用计数是原子操作，可以用在多线程中。这一点是通过编译器静态检查保证的。
///
/// ```rs
/// impl<T: ?Sized> Deref for Rc<T> {
///     type Target = T;
///
///     #[inline(always)]
///     fn deref(&self) -> &T {
///         &self.inner().value
///     }
/// }
/// ```
///
/// Rc类型重载了“解引用”运算符，恰好Target类型指定的是T。这就意味着编译器可以将`Rc<T>`类型在必要的时候自动
/// 转换为`&T`类型，于是它就可以访问`T`的成员变量，调用`T`的成员方法了。因此，它可以被归类为“智能指针”。
///
#[test]
fn _17_02_01_smart_pointer() {

    use std::rc::Rc;

    struct SharedValue {
        value: i32
    }

    // Rc指针的创建是调用`Rc::new`静态函数，由于Rc指针没有实现Copy trait，需要显式调用clone函数，
    // 如果直接赋值，会执行move语义，导致前一个指针失效，引用计数值不变
    // 必须手工调用clone()函数，此时引用计数值才会加1.当某个Rc指针失效，会导致引用计数值减1。当引用计数值
    // 减到0的时候，共享内存空间才会被释放。
    let shared_value: Rc<SharedValue> = Rc::new(SharedValue { value: 42 } );
    let owner1 = shared_value.clone();
    let owner2 = shared_value.clone();

    println!("value: {} {}", owner1.value, owner2.value);
    println!("address: {:p} {:p}", &owner1.value, &owner2.value);

    // owner1 owner2里面包含的数据不仅值是相同的，而且地址也是相同的。

}

///
/// Rc智能指针内部实现了Clone和Drop，在clone方法，它没有对内部的数据实行深复制，而是将强引用计数值加1
///
/// ```rs
/// impl<T: ?Szied> Clone for Rc<T> {
///     #[inline]
///     fn clone(&self) -> Rc<T> {
///         self.inc_strong();
///         Rc { ptr: self.ptr }
///     }
/// }
/// fn inc_strong(&self) {
///     self.inner().strong.set(self.strong().checked_add(1).unwrap_or_else(|| unsafe { abort() }));
/// }
/// ```
///
/// 在drop方法中，也没有直接把内部数据释放掉，而是将强引用计数值减1，当强引用计数值减到0的时候，才会析构掉共享的
/// 那块数据。当弱引用计数值也减为0的时候，才说明没有任何Rc/Weak指针指向这块内存，它占用的内存才会被彻底释放。
///
/// ```rs
/// unsafe impl<#[may_dangle] T: ?Szied> Drop for Rc<T> {
///     fn drop(&mut self) {
///         unsafe {
///             let ptr = self.ptr.as_ptr();
///             self.dec_strong();
///             if self.strong() == 0 {
///                 // destroy the contained object
///                 ptr::drop_in_place(self.ptr.as_mut());
///                 // remove the implicit "strong weak" pointer now that we've
///                 // destroyed the contents.
///                 self.dec_weak();
///                 if self.weak() == 0 {
///                     Heap.dealloc(ptr as *mut u8, Layout::for_value(&*ptr));
///                 }
///             }
///         }
///     }
/// }
/// ```
///
/// Rc智能指针所指向的数据，内部包含了强引用和弱引用的计数值。由于引用计数器是共享的，需要用Cell包起来。
/// 因此，Rc智能指针的实现，必须使用“内部可变性”功能。
///
/// Rc的使用场景在于：当逻辑上不可变的方法的实现细节又要求某部分成员变量具有可变性的时候，我们可以使用“内部可变性”。
///
#[test]
fn _17_02_02_smart_pointer() {
    use std::rc::Rc;
    use std::cell::RefCell;

    let shared_vec: Rc<RefCell<Vec<isize>>> = Rc::new(RefCell::new(vec![1, 2, 3]));
    let shared1 = shared_vec.clone();
    let shared2 = shared1.clone();

    shared1.borrow_mut().push(4);
    println!("{:?}", shared_vec.borrow());

    shared2.borrow_mut().push(5);
    println!("{:?}", shared_vec.borrow());
}


///
///
/// `Cow` Clone-On-Write
///
/// 它对指向的数据可能“拥有所有权”，或者可能“不拥有所有”
///
/// Cow在标准库中是一个enum：
///
/// ```rs
/// pub enum Cow<'a, B: ?Sized + 'a> where B: ToOwned {
///     /// Borrowed data.
///     Borrowed(&'a B),
///
///     /// Owned data.
///     Owned(<B as ToOwned>::Owned)
/// }
/// ```
/// 它可以是Borrowed或者Owned两种状态，可以通过调用to_mut函数获取所有权。
/// 在这个过程中，它实际上会分配一块新的内存，并将原来Borrowed状态的数据通过调用to_owned()方法构造出一个新的
/// 拥有所有权的对象，然后对这块拥有所有权的内存执行操作。
///
///
#[test]
fn _17_03_01_smart_pointer() {

    use std::borrow::Cow;

    fn remove_spaces(input: &str) -> Cow<str> {
        if input.contains(' ') {
            let mut buf = String::with_capacity(input.len());
            for c in input.chars() {
                if c != ' ' {
                    buf.push(c);
                }
            }
            return Cow::Owned(buf);
        }
        return Cow::Borrowed(input);
    }

    let s1 = "no_spaces_in_string";
    let result1 = remove_spaces(s1);

    let s2 = "spaces in string";
    let result2 = remove_spaces(s2);

    println!("{}\n{}", result1, result2);
}


// Rust中允许一部分运算符可以由用户自定义行为，即“操作符重载”。其中“解引用”是一个非常重要的操作符，它允许重载。
// 而需要提醒大家注意的是，“取引用”操作符，如`&`、`&mut`，是不允许重载的。因此，“取引用”和“解引用”并非对称互补关系。
// `*&T`的类型一定是`T`，而`&*T`的类型未必就是`T`。

// 更重要的是，在某些情况下，编译器帮我们插入了自动deref地调用，简化代码。

// 在Deref的基础上，我们可以封装出一种自定义类型，它可以直接调用其内部的其它类型的成员方法，我们可以把这种类型称为智能指针类型。
