
#[test]
fn _03_02_01_expression() {

    let x = 100;
    let y = 10;
    println!("{} {} {} {} {}", x + y, x - y, x * y, x / y, x % y);

    let num1: u8 = 0b_1010_1010;
    let num2: u8 = 0b_1111_0000;

    println!("{:08b}", !num1);
    println!("{:08b}", num1 & num2);
    println!("{:08b}", num1 | num2);
    println!("{:08b}", num1 ^ num2);
    println!("{:08b}", num1 << 4);
    println!("{:08b}", num1 >> 4);

    fn f1() -> bool {
        println!("Call f1");
        true
    }

    fn f2() -> bool {
        println!("Call f2");
        false
    }

    println!("Bit and: {}\n", f2() & f1());
    println!("Logic and: {}\n", f2() && f1());
    println!("Bit or: {}\n", f1() | f2());
    println!("Bit or: {}\n", f1() || f2());
}

/// 赋值表达式
#[test]
fn _03_02_02_assignment_expression() {
    // 声明局部变量，带mut修饰
    let mut x: i32 = 1;
    // x 是mut绑定，所以可以为它重新赋值
    x = 2;      // 移动语义，可以理解为，将内存中值为2的对象，将其ownership移动到x这个变量

    let a = 1;
    let mut b = 2;
    let c = (b = a);    // 赋值表达式的类型为unit，即为空tuple。因此Rust不允许连续赋值，即不允许x=y=z=1
    println!("{:?}", c);

    let i = 2;
    let mut j = 4;
    j += i;
    j *= i;
    println!("{} {}", i, j);

    // Rust中不支持++、--运算符，请使用+=1、-=1代替
}

/// 语句块表达式
#[test]
fn _03_02_03_statement_expression() {

    // 语句和表达式的区分方式是后面带不带分号(;)
    // 如果带分号，意味着这是一条语句，它的类型是`()`；
    // 如果不带分号，它的类型就是表达式的类型

    let x: () = {println!("Hello.");};
    let y: i32 = {println!("Hello.");5};

    // 同理，在函数中，我们也可以利用这样的特点来写返回值：

    fn my_func() -> i32 {
        //........
        100
    }
}

/// 条件语句
#[test]
fn _03_03_01_condition() {

    fn func(n: i32) {
        if n < 0 {      // 和其它语言不同，即使是一条表达式，也要用大括号包起来
            print!("{} is negative", n);
        } else if n > 0 {
            print!("{} is positive", n);
        } else {
            print!("{} is zero", n);
        }
    }

    // if-else 结构还可以当表达式使用
    let x: i32 = if true { 1 } else { 10 };

    // 基于这个原因，Rust中没有所谓的三元表达式写法
}

/// 循环语句
#[test]
fn _03_03_01_loop() {

    let mut count = 0u32;
    println!("Let's count until infinity!");

    loop {
        count += 1;
        if count == 3 {
            println!("three");
            // 不再执行下面的代码，跳至loop开头继续循环
            continue;
        }
        println!("{}", count);
        if count == 5 {
            println!("OK, that's enough");
            // 跳出循环
            break;
        }
    }
}

/// 带有生命周期标识符
#[test]
fn _03_03_01_loop_with_lifetime_specifier() {
    let mut m = 1;
    let n = 1;
    'a: loop {
        if m < 100 {
            m += 1;
        } else {
            'b: loop {
                if m + n > 50 {
                    println!("break");
                    break 'a;
                } else {
                    continue 'a;
                }
            }
        }
    }
}

#[test]
fn _03_03_01_loop_expression() {
    let v = loop {
        break 10;
    };
    println!("{}", v);
}

/// while 语句是带条件判断的循环语句
#[test]
fn _03_03_01_while() {

    let mut n = 1;
    while n < 101 {
        if n % 15 == 0 {
            println!("fizzbuzz");
        } else if n % 3 == 0 {
            println!("fizz");
        } else if n % 5 == 0 {
            println!("buzz");
        } else {
            println!("{}", n);
        }
        // Increment counter
        n += 1;
    }
}

/// for循环
#[test]
fn _03_03_01_for() {
    let array = &[1, 2, 3, 4, 5];
    for i in array {
        println!("The number is {}", i);
    }
}
