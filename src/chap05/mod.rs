
/// 成员方法
#[test]
fn _05_01_01_trait_method() {

    trait Shape {
        fn area(&self) -> f64;
    }

    // trait 中定义的函数，也称为关联函数(associated function)
    // 函数的第一个参数如果是Self相关的类型，且命名为self(小写s)，这个参数被称为“receiver”(接收者)。
    // 具有receiver参数的函数，我们称为“方法”(method)，可以通过变量实例使用小数点来调用。
    // 没有receiver参数的函数，我们称之为“静态函数”(static function)，可以通过类型加双冒号`::`的方式来调用。
    // 在Rust中，函数和方法没有本质区别。


    // Rust中Self(大写S)和self(小写s)都是关键字，大写S的是类型名，小写s的是变量名。
    // 对于第一个self参数，常见的类型有self: Self、self: &Self、self: &mut Self等类型。
    // 上面的这些类型，可以简写为self、&self、&mut self。
    // self参数只能用在第一个参数的位置。

    trait T {
        fn method1(self: Self);
        fn method2(self: &Self);
        fn method3(self: &mut Self);
    }

    // 上下两种写法是完全一样的
    trait U {
        fn method1(self);
        fn method2(&self);
        fn method3(&mut self);
    }

    // 我们可以为某些具体类型实现(impl)这个trait。

    struct Circle {
        radius: f64,
    }

    impl Shape for Circle {
        // Self 的类型就是 Circle
        // self 的类型就是 &Self, 即&Circle
        fn area(&self) -> f64 {
            // 访问成员变量，需要用 self.rradius
            std::f64::consts::PI * self.radius * self.radius
        }
    }
    let c = Circle { radius: 2f64 };
    // 第一个参数名字是self，可以使用小数点语法调用
    println!("The area is {}", c.area());

    // 针对一个类型，我们可以直接对它impl来增加成员方法，无须trait名字
    impl Circle {
        fn get_radius(&self) -> f64 {
            self.radius
        }
    }

    // 我们可以看作是Circle类型impl了一个匿名的trait。用这种方式定义的方法叫做这个类型的“内在方法”(inherent methods)。

    // self参数甚至可以是Box指针类型self:Box<Self>。

}

/// 让trait的self是指针类型
#[test]
fn _05_01_01_boxing_self() {

    trait Shape {
        fn area(self: Box<Self>) -> f64;
    }

    struct Circle {
        radius: f64,
    }

    impl Shape for Circle {
        // Self 类型就是Circle
        // self 的类型是Box<Self>，即Box<Circle>
        fn area(self: Box<Self>) -> f64 {
            // 访问成员变量，需要有 self.radius
            std::f64::consts::PI * self.radius * self.radius
        }
    }

    let c = Box::new(Circle { radius: 4f64 });
    c.area();
}

/// impl 的对象甚至可以是trait
#[test]
fn _05_01_01_impl_trait() {
    trait Shape {
        fn area(&self) -> f64;
    }

    trait Round {
        fn get_radius(&self) -> f64;
    }

    struct Circle {
        radius: f64,
    }

    impl Round for Circle {
        fn get_radius(&self) -> f64 {
            self.radius
        }
    }
    // 注意这里是impl Trait for Trait
    impl Shape for Round {
        fn area(&self) -> f64 {
            std::f64::consts::PI * self.get_radius() * self.get_radius()
        }
    }

    let c = Box::new(Circle { radius: 4f64 }) as Box<Round>;
    c.area();

    // 这里impl Shape for Round 和 impl<T: Round>Shape for T是不一样的
    // 前一种写法中，self是&Round类型，它是一个trait object，是胖指针。
    // 后一种写法中，self是&T类型，是具体类型。
    // 前一种写法是为trait object增加一个成员方法，
    // 后一种写法是为所有满足T: Round的具体类型增加一个成员方法，
    // 所以上面的示例中，只能构造一个trait object之后才能调用area()成员方法。
}

/// 没有receiver参数的方法(第一个参数不是self参数的方法)称为“静态方法”。
/// 静态方法可以通过Type::FunctionName()的方式调用。
///
/// 需要注意的是，即便第一个参数是Self相关类型，只要变量名不是self，就不能使用小数点的语法调用函数.
#[test]
fn _05_02_01_static_method() {

    struct T(i32);

    impl T {
        // 这是一个静态方法
        fn func(this: &Self) {
            println!("value {}", this.0)
        }
    }

    let x = T(42);
    // x.func();    由于func是个静态方法，不能通过小数点方式调用
    T::func(&x);

    // 标准库中有大量这些例子。
    // Box的一系列方法 Box::into_raw(b: Self)、Box::leak(b: Self)
    // Rc的一系列方法  Rc::try_unwrap(this:Self)、Rc::downgrade(this: &Self)
    // 它们的receiver不是self关键字，这样设计的目的是强制用户用Rc::downgrade(&obj)的形式调用，而禁止obj.downgrade()形式的调用。
    //
}

/// 静态函数
#[test]
fn _05_02_01_static_function() {

    // 无参数的函数，返回类型是实现该trait的具体类型
    trait Default {
        fn default() -> Self;
    }
}

/// 扩展方法
#[test]
fn _05_03_01_method_extension() {

    // 可以利用trait给其它类型添加成员方法

    trait Double {
        fn double(&self) -> Self;
    }

    impl Double for i32 {
        fn double(&self) -> Self { *self * 2 }
    }

    let x: i32 = 10.double();
    println!("{}", x);

    // 在声明trait和impl trait的时候，Rust规定了一个Coherence Rule(一致性规则)或称为Orphan Rule(孤儿规则)：
    // impl块要么与trait的声明在同一个crate中，要么与类型的声明在同一crate中

    // trait本身既不是具体类型，也不是指针类型，它只是定义针对类型的、抽象的“约束”。
    // 不同的类型可以实现同一个trait，满足同一个trait的类型可能具有不同大小
    // 因此，trait在编译阶段没有固定大小，不能直接使用trait作为实例变量、参数、返回值
}


/// 完整函数调用语法
/// Fully Qualified Syntax 提供了一种无歧义的函数调用语法，允许程序员精确指定想调用的是哪个函数。
/// 以前叫UFCS(universal function call syntax)，即“通用函数调用语法”。
/// 它的具体写法为`<T as TraitName>::item`
///
#[test]
fn _05_04_01_ufcs() {

    trait Cook {
        fn start(&self);
    }
    trait Wash {
        fn start(&self);
    }

    struct Chef;

    impl Cook for Chef {
        fn start(&self) { println!("Cook::start") }
    }

    impl Wash for Chef {
        fn start(&self) { println!("Wash::start") }
    }

    // 如果一个类型同时显示了这两个trait，那么如果我们使用variable.start()这样的语法执行方法调用的话，就会出现歧义，
    // 编译器不知道你具体想调用哪个方法，编译错误信息为“multiple applicable items in scope”。

    // ILLEGAL let me = Chef;
    // ILLEGAL me.start();

    // 这时候，需要使用完整的函数调用语法来进行方法调用
    let me = Chef;
    // 函数名字使用更完整的path来指定，同时，self参数需要显式传递
    <Cook>::start(&me);
    <Chef as Wash>::start(&me);
}

/// 函数和方法的调用本质上没有区别
#[test]
fn _05_04_02_method_function() {
    struct T(usize);

    impl T {
        fn get1(&self) -> usize { self.0 }
        fn get2(&self) -> usize { self.0 }
    }

    fn get3(t: &T) -> usize { t.0 }

    fn check_type( _: fn (&T) -> usize ) {}

    check_type(T::get1);
    check_type(T::get2);
    check_type(get3);
}

/// trait约束和继承
#[test]
fn _05_05_01_trait_ad_hoc() {

    use std::fmt::Debug;

    // 泛型约束，要求这个T类型实现Debug
    fn my_print<T: Debug>(x: T) {
        println!("The value is {:?}.", x);
    }

    my_print("China");
    my_print(41_i32);
    my_print(true);
    my_print(['a', 'b', 'c']);

    // 泛型约束的另一种写法
    fn my_other_print<T>(x: T) where T: Debug {
        println!("The value is {:?}", x);
    }

    // 对于简单情况，两种写法都可以。
    // 但是在某些复杂的情况下，泛型约束只有where子句可以表达，泛型参数后面直接加冒号的写法表达不出来


    // trait允许继承
    trait Base {}
    trait Derived: Base {}

    // 所以针对一个具体类型impl Derived，编译器也要求impl Base
    struct T;

    impl Derived for T {}
    impl Base for T {}
}

/// Rust里面为类型impl某些trait的时候，逻辑是非常机械化的。
/// 为许多类型重复而单调地impl某些trait，是非常枯燥的事情。
/// 为此，Rust提供了一个特殊的attribute，它可以帮我们自动impl某些trait
#[test]
fn _05_06_01_derive() {

    #[derive(Copy, Clone, Default, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
    struct Foo {
        data: i32
    }

    let v1 = Foo { data: 0 };
    let v2 = v1;
    println!("{:?}", v2);

    // 它的语法是，在你希望impl trait的类型前面写`#[derive (..) ]`

    // 目前Rust支持的可以自动derive的trait有以下这些

    // Debug Clone Copy Hash
    // RustcEncodable RustcDecodable PartialEq Eq
    // ParialOrd Ord Default
    // FromPrimitive Send Sync

}

/// trait 别名
#[test]
fn _05_07_01_trait_alias() {



    //trait Service {
    //    type Request;
    //    type Response;
    //    type Error;
    //    type Future: Future<Item=Self::Response, Error=Self::Error>;
    //    fn call(&self, req: Self::Request) -> Self::Future;
    //}
    //
    //trait HttpService = Service<Request = http::Request, Response = http::Response, Error = http::Error>;
}

/// 标准库常见trait
#[test]
fn _05_08_01_display_and_debug() {

    use std::fmt::{Display, Formatter, Error};

    #[derive(Debug)]
    struct T {
        field1: i32,
        field2: i32,
    }

    impl Display for T {
        fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
            write!(f, "{{ field1: {}, field2: {} }}", self.field1, self.field2)
        }
    }

    let var = T { field1: 1, field2: 2 };
    println!("{}", var);
    println!("{:?}", var);
    println!("{:#?}", var);
}

/// 全序关系
///
#[test]
fn _05_08_02_ord_eq() {

    // 对于集合X中的元素a,b,c
    // 1. 如果a < b则一定有 !(a>b)；反之，若a > b，则一定有 !(a<b)，称为反对称性。
    // 2. 如果a < b且b < c则a < c，称为传递性。
    // 3. 对于X中的所有元素，都存在a<b或a>b或a==b，三者必居其一，称为完全性。

    // 如果集合X中的元素只具备上述前两条特征，称X是“偏序”。
    // 同时具备以上所有特征，称X是“全序”。

    // 显然浮点数中的特殊值NaN不满足完全性，这导致一个问题：浮点数无法排序。
    // 因此，Rust设计了两个trait来描述这样的状态：
    // 一个是 std::cmp::PartialOrd，表示“偏序”
    // 一个是 std::cmp::Ord，表示“全序”

    let int_vec = [1_i32, 2, 3];
    let biggest_int = int_vec.iter().max();

    let float_vec = [1.0_f32, 2.0, 3.0];
    // ILLEGAL: let biggest_float = float_vec.iter().max();

    let biggest_float = float_vec.iter().fold(std::f32::NEG_INFINITY, |a, &b| a.max(b));

    println!("{:?} {:?}", biggest_int.unwrap(), biggest_float);

}

/// Sized
/// Sized trait是Rust中一个非常重要的trait
/// 它定义在std::marker模块中，没有任何成员方法。
/// 它有`#[lang="sized"]`属性，它是lang级别的，用户不能impl这trait。
/// 一个类型是否满足Sized约束完全由编译器推导的，用户无权指定
#[test]
fn _05_08_03_sized() {

}

/// Rust中没有“构造函数”的概念。因为，相比普通函数，构造函数本身并没有提供什么额外的抽象能力。
/// 所以默认所谓的“默认构造函数”，但可以通过impl Default来实现默认值。
#[test]
fn _05_08_04_default() {

    // 标准库提供了Default trait来处理无参数、无错误处理的“默认值”
    trait Default {
        fn default() -> Self;
    }

    // 标准库中很多类型都实现了这个trait，它相当于提供了一个类型的默认值。
    // Rust中，单词new并不是关键字，
}