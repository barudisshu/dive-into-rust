
///
/// 生命周期
///
#[test]
fn _13_01_01_lifetime() {

    // v 的生命周期开始
    let v = vec![1, 2, 3];
    {
        let center = v[2];
        println!("{}", center);
    }
    println!("{:?}", v);
}

///
/// 借用(borrow)
/// 变量对其管理的内存拥有所有权。这个所有权不仅可以被转移（move），还可以被借用（borrow）
/// 借用指针的语法使用&符号或者&mut符号表示。前者表示只读借用，后者表示可读写借用。
/// 借用指针（borrow pointer）也可以称作“引用”（reference）。
/// 借用指针与普通指针的内部数据是一模一样的，唯一的区别是语义层面上的。它的作用是告诉编译器，它对指向的这块内存区域没有所有权。
///
#[test]
fn _13_01_02_borrow() {

    fn foo(v: &mut Vec<i32>) {
        v.push(5);
    }

    // 需要这个动态数组本身是“可变的”，才能获得它的“可变借用指针”
    let mut v = vec![];
    // 在函数调用的时候，需要获取它的“可变借用指针”
    foo(&mut v);
    assert_eq!(5, v[0]);
}

///
/// `&mut` 指针，和变量绑定
///
#[test]
fn _13_01_03_borrow() {
    let mut var = 0_i32;
    {
        // p1 指针本身不能被重新绑定,我们可以通过p1改变变量var的值
        let p1 = &mut var;
        *p1 = 1;
    }
    {
        let temp = 2_i32;
        // 我们不能通过p2改变变量var的值,但p2指针本身指向的位置可以被改变
        let mut p2 = &var;
        p2 = &temp;
    }
    {
        let mut temp = 3_i32;
        // 我们既可以通过p3改变变量var的值,而且p3指针本身指向的位置也可以改变
        let mut p3 = &mut var;
        *p3 = 3;
        p3 = &mut temp;
    }
}

///
/// 借用规则
///
/// - 借用指针不能比它指向的变量存活更长
/// - `&mut`的借用只能指向本身具有`mut`修饰的变量，对于只读变量，不能用`&mut`修饰
/// - `&mut`型借用指针存在的时候，被借用的变量本身会处于“冻结”状态
/// - 如果只有`&`型借用指针，那么能同时存在多个；如果存在`&mut`型指针，那么只能存在一个
///
/// 借用指针只能临时地拥有对这个变量读或写的权限，没有义务管理这个变量的生命周期。因此，借用指针的生命周期
/// 绝对不能大于它所引用变量的生命周期，否则就是悬空指针，导致内存不安全。
///
/// 解决悬空指针的办法是引入`lifetime specifier`，它是个特殊的annotation，和类型参数写法一致。
///
#[test]
fn _13_02_01_borrow_rule() {
    // 这里的参数采用的“引用传递”,意味着实参本身并未丢失对内存的管理权
    fn borrow_semantics(v: &Vec<i32>) {

        // 打印参数占用空间的大小,在64位系统上,结果为8,表明该指针与普通裸指针的内部表示方法相同
        println!("size of param: {}", std::mem::size_of::<&Vec<i32>>());
        for item in v {
            print!("{} ", item);
        }
        println!("");
    }
    // 这里的参数采用的“值传递”,而Vec没有实现Copy trait,意味着它将执行move语义
    fn move_semantics(v: Vec<i32>) {

        // 打印参数占用空间的大小,结果为24,表明实参中栈上分配的内存空间复制到了函数的形参中
        println!("size of param: {}", std::mem::size_of::<Vec<i32>>());
        for item in v {
            print!("{} ", item);
        }
        println!("")
    }

    let array = vec![1, 2, 3];

    // 需要注意的是,如果使用引用传递,不仅在函数声明的地方需要使用&标记
    // 函数调用的地方同样需要使用&标记,否则会出现语法错误
    // 这样设计主要是为了显眼,不用去阅读该函数的签名就知道这个函数调用的时候发生了什么
    // 而小数点方式的成员函数调用,对于self参数,会“自动转换”,不必显式借用,这里有个区别
    borrow_semantics(&array);

    // 在使用引用传递给上面的函数后,array本身依然有效,我们还能在下面的函数中使用
    move_semantics(array);

    // 在使用move语义传递后,array在这个函数调用后,它的生命周期已经完结

}

///
/// 一般情况下，函数参数使用引用传递的时候，不仅在函数声明这里要写上类型参数，
/// 在函数调用这里也要显式地引用运算符。
///
/// 但是，有个例外，当参数为`self` `&self` `&mut self`等时，若使用小数点语法调用成员方法，
/// 在函数调用这里不能显式写出借用运算符。
///
#[test]
fn _13_02_02_borrow_rule() {
    // 创建一个可变的String 类型实例
    let mut x: String = "hello".into();
    // 调用`len(&self) -> usize`函数。`self`的类型是`&Self`
    // `x.len()`等同于`String::len(&x)`
    assert_eq!(6, x.len());
    // 调用`fn push(&mut self, ch: char)`函数。`self`的类型是`&mut Self`，因此它有权对字符串做修改
    // `x.push('!')`等同于`String::push(&mut x, '!')`
    x.push('!');
    assert_eq!(7, x.len());

    // 调用`fn into_bytes(self) -> Vec<u8>`函数。注意self类型，此处发生了所有权转移
    // `x.into_bytes()`等同于`String::into_bytes(x)`
    let v = x.into_bytes();

    // 再次调用`len()`，编译失败，因为此处已经超过了x的生命周期
    // ILLEGAL: assert_eq!(7, x.len());
}


///
/// 任何借用指针的存在，都会导致原来变量被“冻结(Frozen)”
///
#[test]
fn _13_02_03_borrow_rule() {
    let mut x = 1_i32;
    let p = &mut x;
    // ILLEGAL: x = 2;
    assert_eq!(1, *p);

    // 因为p的存在，此时对x的改变被认为是非法的
}


///
/// 生命周期标识符
/// 借用指针类型都有一个泛型参数，它的完整写法应该是`&'a T` `&'a mut T`，只不过做局部变量的时候，生命周期参数可以省略
///
/// 生命周期之间有重要的包含关系，如果生命周期`'a`比`'b`更长或相等，记做`'a: 'b`，意思是`'a`至少不会比`'b`短，
/// 读作'lifetime a outlives lifetime b'。对于借用指针来说，如果`&'a`合法，那么`'b`作为`'a`的一部分，`&'b`也一定是
/// 合法的。
///
/// 另外，`'static`是一个特殊的生命周期，它代表的是这个程序从开始到结束的整个阶段，所以它比其他任何生命周期都长。
/// 这意味着，任意一个生命周期`'a`都满足`'static: 'a`。
///
#[test]
fn _13_03_01_lifetime_specifier() {

    struct T {
        member: i32,
    }

    fn test<'a>(arg: &'a T) -> &'a i32 {
        &arg.member
    }

    let t = T { member: 0 };
    let x = test(&t);
    assert_eq!(0, *x);
}


#[test]
fn _13_03_02_lifetime_specifier() {
    struct T {
        member: i32,
    }
    // 指定`'a: 'b`，要求`'a`要比`'b`活的长，这样`&'a i32`类型赋值给`&'b i32`类型就没有问题
    fn test<'a, 'b>(arg: &'a T) -> &'b i32 where 'a: 'b {
        &arg.member
    }
    let t = T { member: 0 };
    let x = test(&t);
    assert_eq!(0, *x);
}

///
/// Rust的引用类型是支持“协变”的。
/// 在编译期眼里，生命周期就是一个区间，生命周期参数就是一个普通的泛型参数，它可以被特化为某个具体的生命周期。
///
#[test]
fn _13_03_03_lifetime_specifier() {
    fn select<'a>(arg1: &'a i32, arg2: &'a i32) -> &'a i32 {
        if *arg1 > *arg2 {
            arg1
        } else {
            arg2
        }
    }
    let x = 1;
    let y = 2;
    let selected = select(&x, &y);
    assert_eq!(2, *selected);
}

///
/// 类型的生命周期标记
///
#[test]
fn _13_04_01_type_lifetime_specifier() {
    struct Test<'a> {
        member: &'a str,
    }

    // `impl`时，也要声明再使用
    impl<'t> Test<'t> {
        fn test<'a>(&self, s: &'a str) {

        }
    }
}

///
/// 省略生命周期标记
///
/// “lifetime elision rules”
///
/// - 每个带生命周期参数的输入参数，每个对应不同的生命周期参数；
/// - 如果只有一个输入参数带生命周期参数，那么返回值的生命周期被指定为这个参数；
/// - 如果有多个输入参数带生命周期参数，但其中有`&self`，`&mut self`那么返回值的生命周期被指定为这个参数；
/// - 以上都不满足，就不能自动补全返回值的生命周期参数。
///
#[test]
fn _13_04_02_type_lifetime_specifier() {

    fn get_str(s: &String) -> &str {
        println!("call fn {}", s);
        "hello world"
    }

    // 编译器会自动补全生命周期参数
    fn get_str1<'a>(s: &'a String) -> &'static str {
        println!("call fn {}", s);
        "hello world"
    }

}