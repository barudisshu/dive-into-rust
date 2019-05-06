///
/// 闭包
///
///
#[test]
fn _23_01_01_closure() {
    let add = |a: i32, b: i32| -> i32 { return a + b; };
    let x = add(1, 2);
    println!("result is {}", x);
}

#[test]
fn _23_01_02_closure() {
    let add = |a, b| -> i32 { return a + b; };
    let x = add(1, 2);
    println!("result is {}", x);
}

#[test]
fn _23_01_03_closure() {
    let add = |a, b| a + b;
    let x = add(1, 2);
    println!("result is {}", x);
}

///
/// 闭包可以捕获自由变量
///
/// - 如果一个外部变量在闭包中，值通过借用指针`&`使用，那么这个变量就可通过引用`&`的方式捕获；
/// - 如果一个外部变量在闭包中，通过`&mut`指针使用过，那么这个变量就需要使用`&mut`的方式捕获；
/// - 如果一个外部变量在闭包中，通过所有权转移的方式使用过，那么这个变量就需要使用"by value"self的方式捕获。
///
#[test]
fn _23_01_04_closure() {

    let x = 1_i32;
    let inner_add = || x + 1;
    let x2 = inner_add();
    println!("result is {}", x2);
}

#[test]
fn _23_01_05_closure() {
    struct T(i32);

    fn by_value(_: T) {}
    fn by_mut(_: &mut T) {}
    fn by_ref(_: &T) {}

    let x: T = T(1);
    let y: T = T(2);
    let mut z: T = T(3);

    let closure = || {
        by_value(x);
        by_ref(&y);
        by_mut(&mut z);
    };

    closure();

    // 实际类型名字是编译器按某些规则自动生成的
    struct ClosureEnvironment<'y, 'z> {
        x: T,
        y: &'y T,
        z: &'z mut T,
    }
}

///
/// move 关键字
///
#[test]
fn _23_02_01_move() {

    fn make_adder(x: i32) -> Box<Fn(i32) -> i32> {
        Box::new(move |y| x + y)
    }

    let f = make_adder(3);
    println!("{}", f(1));
    println!("{}", f(10));

    // 加上move关键字之后，所有的变量捕获全部使用by value的方式。也就是说，编译器生成的
    // 匿名结构体内部看起来像是下面这样：

    type TYPE1 = i32;
    type TYPE2 = i32;
    type TYPE3 = i32;

    struct ClosureEnvironment {
        x: TYPE1,  //
        y: TYPE2,  // 这里没有 &TYPE,&mut TYPE,所有被捕获的外部变量所有权一律转移进闭包
        z: TYPE3,  //
    }
}

///
/// FnOnce 仅能被调用一次
/// Fn
///
#[test]
fn _23_02_02_move() {
    let v: Vec<i32> = vec![1, 2, 3];
    let c = move || for i in &v {println!("{}", i); };  // Fn可以被多次调用
    c();
    c();
}

///
/// 每个闭包，编译器都会为它生成一个匿名结构体类型；即使两个闭包的参数和返回值一致，
/// 它们也是完全不同的两个类型，只是都实现了同一个trait而已。
///
#[test]
fn _23_04_01_closure() {
    fn call_with_closure<F>(some_closure: F) -> i32 where F: Fn(i32) -> i32 {
        some_closure(1)
    }
    let answer = call_with_closure(|x| x + 2);
    println!("{}", answer);
}

///
/// 动态指派
///
#[test]
fn _23_04_02_dynamic_assign() {
    fn static_dispatch<F>(closure: &F) where F: Fn(i32) -> i32 {
        println!("static dispatch {}", closure(42));
    }

    fn dynamic_dispatch(closure: &Fn(i32) -> i32) {
        println!("dynamic dispatch {}", closure(42));
    }

    let closure1 = |x| x * 2;
    let closure2 = |x| x * 3;
    fn function_ptr(x: i32) -> i32 { x * 4 };

    static_dispatch(&closure1);
    static_dispatch(&closure2);
    static_dispatch(&function_ptr); // 普通`fn`函数也实现了`Fn trait`,它可以与此参数类型匹配。`fn`不可以捕获外部变量

    dynamic_dispatch(&closure1);
    dynamic_dispatch(&closure2);
    dynamic_dispatch(&function_ptr);

    fn test() -> Box<dyn Fn(i32) -> i32> {
        let c = |i: i32| i * 2;
        Box::new(c)
    }

    let closure3 = test();
    let r = closure3(2);
    println!("{}", r);
}

///
/// 闭包生命周期
///
/// 当使用闭包做参数或返回值的时候，生命周期会变得更加复杂。
#[test]
fn _23_04_03_closure_lifetime() {

    // “高阶生命周期”的表示方式
    // 到目前为止，`for<'a>Fn（&'a Arg）->&'a Ret`这样的语法，只能用于生命周期参数，不能用于任意泛型类型。
    fn calc_by<'a, F>(var: &'a i32, f: F) -> i32 where F: for <'f> Fn(&'f i32) -> i32 {
        let local = *var;
        f(&local)
    }
}

























