
#[test]
fn _04_01_01_function() {

    fn add1(t: (i32, i32)) -> i32 {
        t.0 + t.1
    }

    fn add2((x,y): (i32, i32)) -> i32 {
        x + y
    }

    // 函数不写返回值的情况下，默认是`unit()`

    let p = (1, 3);

    let func = add2;
    // func 可以被当成普通函数一样被调用
    println!("evaluation output {}", func(p));

    // 先让 func 指向 add1
    let mut func = add1;

    // 再重新赋值，让func 指向add2
    // ILLEGAL: func = add2;

    // 虽然add1和add2有同样的参数类型和同样的返回值类型，但它们是不同类型，所以这里报错了。

    // 写法一，用as类型转换
    let mut func = add1 as fn((i32, i32)) -> i32;
    // 写法二，用显式类型标记
    let mut func: fn((i32, i32))->i32 = add1;

}

#[test]
fn _04_01_01_function_body() {
    static INNER_STATIC: i64 = 42;

    // 函数内部定义函数
    fn internal_incr(x: i64) -> i64 {
        x + 1
    }

    struct InnerTemp(i64);

    impl InnerTemp {
        fn incr(&mut self) {
            self.0 = internal_incr(self.0);
        }
    }

    // 函数体，执行语句
    let mut t = InnerTemp(INNER_STATIC);
    t.incr();
    println!("{}", t.0);
}

/// 发散函数
/// Rust支持一种特殊的发散函数(Diverging functions)，它的返回类型是感叹号`!`。
#[test]
#[should_panic]
fn _04_02_01_diverging_functions() {

    // 如果一个函数根本就不能正常返回，那么它可以这样写：
    fn diverges() -> ! {
        panic!("This function never returns!");
    }

    // 因为`panic!`会直接导致栈展开，所以这个函数调用后面的代码都不会继续执行，它的返回类型就是一个
    // 特殊的`!`符号，这种函数也叫发散函数。发散函数的最大特点就是，它可以被转换为任意一个类型。譬如，

    let x: i32 = diverges();
    let y: String = diverges();

    // Rust中以下情况的返回类型是`!`

    // `panic!`以及基于它实现的各种函数/宏，比如`unimplemented!`、`unreachable!`；
    // 死循环`loop {}`；
    // 进程退出函数`std::process::exit`以及类似的libc中的exec一类函数
}

/// const fn
/// 函数可以用`const`关键字修饰，这样的函数可以在编译阶段被编译器执行，返回值也被视为编译期常量
#[test]
fn _04_04_01_const_fn() {

    #![feature(const_fn)]
    const fn cube(num: usize) -> usize {
        num * num * num
    }

    const DIM: usize = cube(2);
    const ARR: [i32; DIM] = [0; DIM];
    println!("{:?}", ARR);
}

/// 递归函数
#[test]
fn _04_05_01_recursive() {

    fn fib(index: u32) -> u64 {
        if index == 1 || index == 2 {
            1
        } else {
            fib(index - 1) + fib(index - 2)
        }
    }

    let f8 = fib(8);
    println!("{}", f8);

    // 当前版本的Rust暂时还不支持尾递归优化，
    // 因此，如果递归调用层次太多的话，是有可能撑爆栈空间的
}