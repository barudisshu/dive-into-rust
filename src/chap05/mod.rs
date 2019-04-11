
/// 成员方法
pub fn _05_01_01_trait_method() {

    trait Shape {
        fn area(&self) -> f64;
    }

    // trait 中定义的函数，也称为关联函数(associated function)
    // 函数的第一个参数如果是Self相关的类型，且命名为self(小写s)，这个参数被称为“receiver”(接收者)。
    // 具有receiver参数的函数，我们称为“方法”(method)，可以通过变量实例使用小数点来调用。
    // 没有receiver参数的函数，我们称之为“静态函数”(static function)，可以通过类型加双冒号`::`的方式来调用。
    // 在Rust中，函数和方法没有本质区别。


}