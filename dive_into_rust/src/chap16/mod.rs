//!
//! 内部可变性(interior mutability)
//!
//! Rust的borrow checker的核心思想是“共享不可变，可变不共享”
//!
//! “内部可变性”的概念，是与“承袭可变性”（inherited mutability）相对应的。
//!
//! 可变还是不可变取决于变量的使用方式，这就叫作“承袭可变性”。
//!
//! 如果我们用`let var: T;` 声明，那么`var`是不可变的，同时`var`内部的所有成员都是不可变的；
//! 如果我们用`let mut var: T;`声明，那么`var`是可变的，相应的，它的内部所有成员也都是可变的。
//!
//! 我们不能在类型声明的时候指定可变性，比如在struct中对某部分成员使用mut修饰，这是不合法的。我们只能在变量
//! 声明的时候指定可变性。我么也不能针对变量的某一部分成员指定可变性，其他部分保持不变。
//!
//! 常见的具备内部可变性特点的类型有：
//! `Cell` `RefCell` `Mutex` `RwLock` `Atomic*`等。其中`Cell`和`RefCell`是只能用在单线程环境下的具备内部可变性的类型。
//!
//!
//!

///
/// `Cell`，
/// 如果我们有共享引用指向一个对象，那么这个对象就不会被更改了。因为在共享引用存在的期间，不能有可变引用同时
/// 指向它，因此它一定是不可变的。
#[test]
fn _16_01_01_interior_mutability() {


    use std::rc::Rc;
    let r1 = Rc::new(1);
    println!("reference count {}", Rc::strong_count(&r1));
    let r2 = r1.clone();
    println!("reference count {}", Rc::strong_count(&r2));
}

///
/// 可变类型可以通过共享指针修改它内部的值
///
/// `Cell`类型把数据包裹在内部，用户无法获得指向内部状态的指针，意味着每次方法调用都是执行一次完整的数据移动操作。
/// 每次方法调用后，`Cell`类型的内部都处于一个正确的状态，我们不可能观察到数据被破坏的状态。
///
/// 多个共享指针指向`Cell`类型的状态类似于，
///
///```txt
///
///   共享引用              共享引用              共享引用
///      ↓                    ↓                    ↓
/// ====================== Cell ===================================
///                     (内部可变性)
///                ╭──────────────────╮
///                |      内部数据     |
///                ╰──────────────────╯
///
/// ```
///
/// `Cell`就是一个“壳”，它把数据严严实实地包裹在里面，所有指针只能指向`Cell`，不能直接指向数据。
/// 修改数据只能通过`Cell`来完成，用户无法创造一个直接指向数据的指针。
///
///
/// ```
/// impl<T> Cell<T> {
///     pub fn get_mut(&mut self) -> &mut T {}
///     pub fn set(&self, val: T) {}
///     pub fn swap(&self, other: &Self) {}
///     pub fn replace(&self, val: T) -> T {}
///     pub fn intro_inner(self) -> T {}
/// }
/// impl<T: Copy> Cell<T> {
///     pub fn get(&self) -> T {}
/// }
/// ```
///
/// - `get_mut`方法可以从`&mut Cell<T>`类型制造出一个`&mut T`型指针。因为`&mut`型指针具有“独占性”，
/// 所以这个函数保证了调用前，有且仅有一个“可写“指针指向`Cell`，调用后有且仅有一个“可写”指针指向内部
/// 数据。它不存在制造多个引用指向内部数据的可能性。
/// - `set`方法可以修改内部数据。它是把内部数据整个替换掉，不存在多个引用指向内部数据的可能性。
/// - `swap`方法也是修改内部数据。跟`set`方法一样，也是把内部数据整体替换掉。与`std::mem::swap`函数的
/// 区别在于，它仅要求`&`引用，不要求`&mut`引用。
/// - `replace`方法也是修改内部数据。跟`set`方法一样，它也是把内部数据整体替换，唯一区别是，换出来的数据
/// 作为返回值返回了。
///- `into_inner`方法相当于把这个“壳”剥掉了。它接受的是`Self`类型，即move语义，原来的`Cell`类型的变量
/// 会被move进入这个方法，会把内部数据整体返回出来。
/// - `get`方法接受的是`&self`参数，返回的是`T`类型，它可以在保留之前`Cell`类型不变的情况下返回一个新的
/// `T`类型变量，因此它要求`T: Copy`约束。每次调用它的时候，都相当于把内部数据`memcpy`了一份返回出去。
///
#[test]
fn _16_01_02_interior_mutability() {
    use std::cell::Cell;

    // data 这个变量绑定没有用mut修饰，p这个指针也没有用`&mut`修饰
    let data: Cell<i32> = Cell::new(100);
    let p = &data;
    // 不可变引用竟然可以调用set函数，改变了变量的值，没有出现编译错误
    data.set(10);
    println!("{}", p.get());

    p.set(20);
    println!("{:?}", data);
}

///
/// `RefCell`
///
/// `RefCell`是另外一个提供了内部可变性的类型。它提供的方式与`Cell`类型有点不一样。`Cell`类型没办法制造出直接
/// 指向内部数据的指针，而`RefCell`可以。
///
/// ```
/// impl<T: ?Sized> RefCell<T> {
///     pub fn borrow(&self) -> Ref<T> {}
///     pub fn try_borrow(&self) -> Result<Ref<T>, BorrowError> {}
///     pub fn borrow_mut(&self) -> RefMut<T> {}
///     pub fn try_borrow_mut(&self) -> Result<RefMut<T>, BorrowMutError> {}
///     pub fn get_mut(&mut self) -> &mut T {}
/// }
/// ```
///
/// `get_mut`方法与`Cell::get_mut`一样，可以通过`&mut self`获得`&mut T`，这个过程是安全的。
/// 除此之外，`RefCell`最主要的两个方法就是`borrow`和`borrow_mut`，另外两个`try_borrow`和`try_borrow_mut`
/// 只是它们俩的镜像版，区别仅在于错误处理的方式不同。
///
#[test]
fn _16_01_03_interior_mutability() {
    use std::cell::RefCell;

    let shared_vec: RefCell<Vec<isize>> = RefCell::new(vec![1, 2, 3]);
    let shared1 = &shared_vec;
    let shared2 = &shared1;

    shared1.borrow_mut().push(4);
    println!("{:?}", shared_vec.borrow());

    shared2.borrow_mut().push(5);
    println!("{:?}", shared_vec.borrow());

    // `borrow`方法和`borrow_mut`方法返回的并不是`&T`和`&mut T`，而是`Ref<T>`和`RefMut<T>`。它们
    // 实际上是一种“智能指针”，完全可以当做`&T`和`&mut T`的等价物来使用。
}

///
///
///```txt
///                                        borrow()
///                                           |
///   共享引用              共享引用            |  共享引用
///      ↓                    ↓               |    ↓
/// ====================== RefCell ===========|=======================
///                     (读写引用计数)          |
///                ╭──────────────────╮       |
///                |      内部数据     | ------┘
///                ╰──────────────────╯
///
/// ```
///
/// `RefCell`内部有一个“借用计数器”，调用`borrow`方法的时候，计数器里面的“共享引用计数”值就加1。当这个borrow
/// 结束的时候，会将这个值自动减1。同样，`borrow_mut`方法被调用的时候，它就记录一下当前存在“可变引用”。如果
/// “共享引用”和“可变引用”同时出现了，就会报错。
///
#[test]
fn _16_01_04_interior_mutability() {
    use std::cell::RefCell;

    let shared_vec: RefCell<Vec<isize>> = RefCell::new(vec![1, 2, 3]);
    let shared1 = &shared_vec;
    let shared2 = &shared1;

    let p1 = shared1.borrow();
    let p2 = &p1[0];

    shared2.borrow_mut().push(4);
    println!("{}", p2);

    {
        // 先调用`borrow`方法，并制造一个指向数组第一个元素的指针，
        // 接着在调用`borrow_mut`放啊，修改这个数组。构造出同时存在“共享(alias)”和“可变(mutation)”的场景
    }

    {
        // `Cell`和`RefCell`用得最多的场景是和多个只读引用相配合。比如，多个`&`引用或`Rc`引用指向同一个
        // 变量的时候。我们不能直接通过这些只读引用修改变量，因为既然存在alias，就不能提供mutation。为了
        // 让存在多个alias共享的变量也可以被修改，那我们就需要使用内部可变性。
        // Rust中提供了只读引用的类型有`&`、`Rc`、`Arc`等指针，它们可以提供alias。
        // Rust中提供了内部可变性的类型有`Cell`、`RefCell`、`Mutex`、`RwLock`以及`Atomic*`系列类型等。

        // 如果只需要整体性地存入、取出`T`，那么就选`Cell`
        // 如果需要有个可读写指针指向这个`T`修改它，那么就选`RefCell`
    }
}

///
/// `UnsafeCell`
///
/// 所有具有内部可变性特点的类型都必须基于`UnsafeCell`来实现，否则必然出现各种问题。
/// 这个类型是唯一合法的将`&T`类型转为`&mut T`类型的办法。绝对不允许把`&T`直接转换为`&mut T`而获得可变性。
///
///
#[test]
fn _16_01_05_interior_mutability() {

}
