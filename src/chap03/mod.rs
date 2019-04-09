
pub fn _03_02_01_expression() {

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
pub fn _03_02_02_assignment_expression() {
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
    y *= i;
    println!("{} {}", i, y);

    // Rust中不支持++、--运算符，请使用+=1、-=1代替
}

/// 语句表达式
pub fn _03_02_03_statement_expression() {

}