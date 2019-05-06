//!
//!
//! 动态分配(dyn)与静态分配(static)
//!
//!
//! 所谓“静态分派”，是指具体调用哪个函数，在编译阶段就确定下来了。Rust中的“静态分派”靠泛型以及impl trait来完成。
//! 对于不同的泛型类型参数，编译器会生成不同版本的函数，在编译阶段就确定好了应该调用哪个函数。
//!
//! 所谓“动态分派”，是指具体调用哪个函数，在执行阶段才能确定。Rust中的“动态分派”靠Trait Object来完成。
//! Trait Object本质上是指针，它可以指向不同的类型；指向的具体类型不同，调用的方法也就不同。
//!


use crate::chap24::Foo;

///
///
///
#[test]
fn _24_01_01_dispatch() {
    trait Bird {
        fn fly(&self);
    }

    struct Duck;
    struct Swan;

    impl Bird for Duck {
        fn fly(&self) { println!("duck duck"); }
    }

    impl Bird for Swan {
        fn fly(&self) { println!("swan swan");}
    }

    // trait是一种DST类型，它的大小在编译阶段是不固定的，所以下面的代码无法编译通过
    // ILLEGAL: fn test(arg: Bird) {}
    // ILLEGAL: fn test() -> Bird  {}

    // “静态分配”的方式
    fn test1<T: Bird>(arg: T) {
        arg.fly();
    }

    // 动态分配的方式
    // 根据不同需求,可以用不同的指针类型,如 Box/&/&mut 等
    fn test2(arg: Box<dyn Bird>) {
        arg.fly();
    }

    // test函数的参数既可以是Box<Duck>类型，也可以是Box<Swan>类型，一样实现了“多态”。
    // 但在参数类型这里已经将“具体类型”信息抹掉了，我们只知道它可以调用Bird trait的方法。
    // 而具体调用的是哪个版本的方法，实际上是由这个指针的值来决定的。
}

///
/// 什么是trait object呢？指向trait的指针就是trait object。
/// 假如Bird是一个trait的名称，那么dyn Bird就是一个DST动态大小类型。
///
/// `&dyn Bird`、`&mut dyn Bird`、`Box<dyn Bird>`、`*const dyn Bird`、`*mut dyn Bird`、`Rc<dyn Bird>`等等都是Trait Object。
///
/// 当指针指向trait的时候，这个指针就不是一个普通的指针了，变成了一个“胖指针”。
///
#[test]
fn _23_01_01_trait_object() {
    use std::mem;

    trait Bird {
        fn fly(&self);
    }

    struct Duck;
    struct Swan;

    impl Bird for Duck {
        fn fly(&self) {
            println!("duck duck");
        }
    }

    impl Bird for Swan {
        fn fly(&self) {
            println!("swan swan");
        }
    }

    // 参数是 trait object 类型，p 是一个胖指针
    fn print_traitobject(p: &dyn Bird) {
        // 使用transmute执行强制类型转换，把变量p的内部数据取出来
        let (data, vtable): (usize, * const usize) = unsafe {
            mem::transmute(p)
        };
        println!("TraitObject [data:{}, vtable:{:p}]", data, vtable);
        unsafe {
            // 打印出指针 v 指向的内存区间的值
            println!("data in vtable [{}, {}, {}, {}]",
                     *vtable,
                     *vtable.offset(1),
                     *vtable.offset(2),
                     *vtable.offset(3));
        }
    }

    let duck = Duck;
    let p_duck = &duck;
    let p_bird = p_duck as &dyn Bird;
    println!("Size of p_duck {}, Size of p_bird {}",
             mem::size_of_val(&p_duck),
             mem::size_of_val(&p_bird));

    let duck_fly: usize = Duck::fly as usize;
    let swan_fly: usize = Swan::fly as usize;
    println!("Duck::fly {}", duck_fly);
    println!("Swan::fly {}", swan_fly);

    print_traitobject(p_bird);
    let swan = Swan;
    print_traitobject(&swan as &dyn Bird);
}

///
/// object safe
///
///
#[test]
fn _23_02_01_object_safe() {

    trait Foo where Self: Sized {
        fn foo(&self);
    }

    impl Foo for i32 {
        fn foo(&self) {
            println!("{}", self);
        }
    }

    let x = 1_i32;
    x.foo();
    // let p = &x as &dyn Foo;
    // p.foo();
}
































