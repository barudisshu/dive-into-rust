///类型系统
///
/// Rust的类型系统实际上是一种代数类型系统(Algebraic data type)。它在数学上有严格定义的，非常严谨的一套理论。
///
/// 1. 理解ADT(algebraic data type)；
///
/// algebraic data type 是用来做模式解构的(pattern destructure)，它是一个组合类型(composite type),
///
/// 分为两大类：product 和 sum,
///
/// sum类是 alternation， `A|B`， 要么是A，要么是B，例如Result(要么是Ok，要么是Err)、Option、Either...
/// product类是 combination，`AB`， 包含`(A,B,C,...)`，例如Tuple1, Tuple2, Product1... struct(case class), Node, Tree,
///
/// `struct { a: A, b: B }`这些`a`, `b`被称为`tag`，用来做模式匹配的“标签”
///
/// 所以sum类型也叫`tagged unions`
///
/// 有tagged unions的数据结构，可以做范畴论的monad形式，例如`Option`可以`flatmap`, `join`, `bind`, `flattern`
///
/// monad 在函数式编程中的实现，主要是用来控制副作用(side effect)
///

#[test]
fn _08_01_01_algebraic_data_type() {

}

/// Never Type
/// 
#[test]
fn _08_01_02_never_type() {

    fn call_fn<T, F: Fn(i32) -> T> (f: F, arg: i32) -> T { f (arg) }
    // 如果不把`!`当成一个类型，那么下面这句话会出现编译错误，因为只有类型才能替换类型参数
    call_fn(std::process::exit, 0);
}

#[test]
fn _08_01_03_never_type() {
    let t = std::thread::spawn(||panic!("nope"));
    t.join().unwrap();
    println!("hello");
    // println永远不可能执行，因为`t.join().unwrap()`会产生一个`!`类型
}

#[test]
fn _08_01_04_never_type() {
    use std::str::FromStr;
    use std::mem::{size_of, size_of_val};

    struct T(String);

    impl FromStr for T {
        type Err = !;

        fn from_str(s: &str) -> Result<T, !> {
            Ok(T(String::from(s)))
        }
    }

    let r: Result<T, !> = T::from_str("hello");
    println!("Size of T: {}", size_of::<T>());
    println!("Size of Result: {}", size_of_val(&r));
    // 将来甚至应该可以直接用let语句进行模式匹配而不发生编译错误
    // 因为编译器有能力推理出Err分支没必要存在
    // let Ok(T(ref s)) = r;
    // println!("{}", s);
}






























