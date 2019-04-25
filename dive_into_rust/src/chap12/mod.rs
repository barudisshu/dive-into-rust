///
///
/// destructor这一词不应该翻译为“析构函数”，因为与之对应的概念是“构造函数”。Rust中并没有“构造函数”的概念，但仍然会被
/// 认为对象的内存分配是“构造”，对象内存的回收是“析构”。这一说法不完全错误。
///
/// 这里我把destructor认定为“销毁器”。类似于迭代器(iterator)、生产者(producer)、迭代生成器(generator)、迭代消费者(consumer)这一类的概念
///
/// Rust中编写“销毁器”的办法是impl `std::ops::Drop`
///
#[test]
fn _12_01_01_destructor() {
    use std::ops::Drop;

    // `Drop` trait允许在对象即将消亡之时，自行调用指定代码。我们来写一个自带析构函数的类型。
    struct D(i32);
    impl Drop for D {
        fn drop(&mut self) {
            println!("destruct {}", self.0);
        }
    }

    let _x = D(1);
    println!("construct 1");
    {
        let _y = D(2);
        println!("construct 2");
        println!("exit inner scope");
    }
    println!("exit main function");

    // 记住一点，变量的声明和销毁是发生在同一个scope的，所以在变量没有被move的情况下，当离开它所在的scope时即发生`drop`操作
    // construct 1
    // construct 2
    // exit inner scope
    // destruct 2
    // exit main function
    // destruct 1
}

///
/// 资源管理
#[test]
fn _12_02_01_source_management() {

    use std::fs::File;
    use std::io::Read;

    let f = File::open("/target/file/path");
    if f.is_err() {
        println!("file is not exist.");
        return;
    }
    let mut f = f.unwrap();
    let mut content = String::new();
    let result = f.read_to_string(&mut content);
    if result.is_err() {
        println!("read file error.");
        return;
    }
    println!("{}", result.unwrap());
}


///
///
/// 主动销毁
///
/// 实现方式很简单，调用`std::mem::drop`函数，或者将所有权move到一个`_`的变量即可，
/// 实际上，
/// `std::mem::drop`函数是最简单的函数，它是一个空方法实现。和move所有权到`_`的操作是等同的
///
#[test]
fn _12_03_01_initiative_destructure() {

    // v的生命周期开始
    let mut v = vec![1, 2, 3];
    // v的生命周期结束
    drop(v);
    // ILLEGAL: v.push(4);


    // 对于有实现`Copy` trait的类型来说，调用`drop`是没有意义的
    let x = 1_i32;
    drop(x);
    assert_eq!(1, x);

    // 下划线变量被当场释放

    use std::ops::Drop;
    struct D(i32);
    impl Drop for D {
        fn drop(&mut self) {
            println!("destructor for {}", self.0);
        }
    }

    let _x = D(1);
    let _  = D(2);  // 被当场释放
    let _y = D(3);


}

///
/// 带有`Copy` trait的类型，不是也是`Drop`的，原因很简单，不能保证内存安全
///
/// 想要实现`Copy` trait，类型必须满足一定条件。这个条件就是：如果一个类型可以使用memcpy的方式执行复制操作，且没有内存
/// 安全问题，才能被允许实现`Copy` trait。
///
#[test]
fn _12_04_01_drop_beyond_copy() {
    use std::ops::Drop;

    struct T;

    impl Drop for T {
        fn drop(&mut self){}
    }

    // 编译就直接报错了the trait `Copy` may not be implemented for this type; the type has a destructor
    // ILLEGAL: impl Copy for T {}

}

///
/// 销毁标记并不是我们需要关心的问题，它通过环境变量标记`export DROP=2`来控制销毁的调用顺序
///
#[test]
fn _12_04_02_destructor_mark() {
    use std::ops::Drop;
    use std::mem::drop;

    struct D(&'static str);
    impl Drop for D {
        fn drop(&mut self) {
            println!("destructor {}", self.0);
        }
    }

    // 获取 DROP 环境变量的值,并转换为整数
    fn condition() -> Option<u32> {
        std::env::var("DROP")
            .map(|s| s.parse::<u32>().unwrap_or(0))
            .ok()
    }
    let var = (D("first"), D("second"), D("third"));
    match condition() {
        Some(1) => drop(var.0),
        Some(2) => drop(var.1),
        Some(3) => drop(var.2),
        _ => {},
    }
    println!("main end");
}
































