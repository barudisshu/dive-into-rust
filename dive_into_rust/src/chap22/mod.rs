//!
//! 高级抽象
//!


use std::fmt::Debug;
use crate::chap22::ConvertTo;

///
/// 泛型
///
#[test]
fn _22_01_01_generic() {
    // 泛型参数可以有多个也可以有默认值
    struct S<T=i32> {
        data: T
    }

    let v1 = S { data: 0 }; // 不指定类型参数，默认为i32
    let v2 = S::<bool> { data: false };

    println!("{} {}", v1.data, v2.data);
}

///
/// 泛型函数
///
/// ```
/// fn contains<'a, P: Pattern<'a>>(&'a self, pat: P) -> bool
/// ```
///
/// 泛型参数满足Pattern trait的约束。意味着，所有实现了Pattern trait的类型，都可以作为参数使用。
///
#[test]
fn _22_01_02_method() {

    fn compare_option<T1, T2>(first: Option<T1>, second: Option<T2>) -> bool {
        match(first, second) {
            (Some(..), Some(..)) => true,
            (None, None) => true,
            _ => false
        }
    }

    println!("{}", compare_option(Some(1i32), Some(1.0f32)));

    // 一般情况下，编译器可以通过类型推倒自动判断。某些时候，确实需要手动指定泛型参数类型。
    // 使用`function_name::<type params>(function params)`的语法

    // 泛型函数基本上实现了“函数重载”功能

    let s = "hello";
    println!("{}", s.contains('a'));
    println!("{}", s.contains("abc"));
    println!("{}", s.contains(&['H'] as &[char]));
    println!("{}", s.contains(|c: char| c.len_utf8() > 2));

    // Rust中没有那种ad hoc式的函数重载，Rust认为这种随意的函数重载对于代码的维护和可读性是一种伤害。
}


/// impl块中的泛型
///
/// ```
///impl<T, U> Into<U> for T where U: From<T> {
///   fn into(self) -> U {
///       u::from(self)
///   }
///}
/// ```
///
#[test]
fn _22_01_03_impl_generic() {

}


///
/// 泛型参数约束有两种方式：
///
/// - 在泛型参数声明的时候使用冒号`:`指定；
/// - 使用where子句指定；
///
#[test]
fn _22_01_04_generic_param_limit() {
    use std::cmp::PartialOrd;

    // 第一种写法
    fn max1<T: PartialOrd>(a: T, b: T) -> T { a }

    // 第二种
    fn max2<T> (a: T, b: T) -> T where T: PartialOrd { a }
}

#[test]
fn _22_01_04_generic_limit_example() {
    use std::cmp::PartialOrd;
    use std::cmp::Ordering;
    use std::fmt::*;

    fn max<T>(a: T, b: T) -> T where T: PartialOrd {
        if a < b {
            b
        } else {
            a
        }
    }

    struct T {
        value: i32,
    }

    impl PartialOrd for T {
        fn partial_cmp(&self, other: &T) -> Option<Ordering> {
            self.value.partial_cmp(&other.value)
        }
    }

    impl PartialEq for T {
        fn eq(&self, other: &T) -> bool {
            self.value == other.value
        }
    }

    impl Debug for T {
        fn fmt(&self, f: &mut Formatter) -> Result {
            write!(f, "T: [ value: {} ]", self.value)
        }
    }

    let t1 = T { value: 1 };
    let t2 = T { value: 2 };
    let m = max(t1, t2);
    println!("the max value is {:?}", m);
}

/// 最直接的方式
#[test]
fn _22_01_04_generic_limit_example_derive() {
    use std::cmp::PartialOrd;
    use std::cmp::Ordering;

    fn max<T>(a: T, b: T) -> T where T: PartialOrd {
        if a < b {
            b
        } else {
            a
        }
    }

    #[derive(PartialOrd, PartialEq, Debug)]
    struct T {
        value: i32,
    }

    let t1 = T { value: 1 };
    let t2 = T { value: 2 };
    let m = max(t1, t2);
    println!("the max value is {:?}", m);
}


///
/// 关联类型(类型投影)
///
/// ```
/// pub trait Iterator {
///    type Item;
///    ...
///}
///```
///
#[test]
fn _22_01_05_associate_type() {
    use std::iter::Iterator;
    use std::fmt::Debug;

    fn use_iter<ITEM, ITER>(mut iter: ITER) where ITER: Iterator<Item=ITEM>, ITEM: Debug {
        while let Some(i) = iter.next() {
            println!("{:?}", i);
        }
    }

    let v: Vec<i32> = vec![1, 2, 3, 4, 5];
    use_iter(v.iter());
}

#[test]
fn _22_01_05_associate_type1() {
    use std::iter::Iterator;
    use std::fmt::Debug;

    fn use_iter<ITER>(mut iter: ITER) where ITER: Iterator, ITER::Item: Debug {
        while let Some(i) = iter.next() {
            println!("{:?}", i);
        }
    }

    let v: Vec<i32> = vec![1, 2, 3, 4, 5];
    use_iter(v.iter());
}

///
/// Rust中不支持泛型参数的重载，所以在调用时需要指定或者给类型实现Pattern trait
///
#[test]
fn _22_01_06_generic_override() {
    trait ConvertTo<T> {
        fn convert(&self) -> T;
    }
    impl ConvertTo<f32> for i32 {
        fn convert(&self) -> f32 {
            *self as f32
        }
    }
    impl ConvertTo<f64> for i32 {
        fn convert(&self) -> f64 {
            *self as f64
        }
    }

    let i = 1_i32;
    let f: f32 = i.convert();
    // 或者
    let f = ConvertTo::<f32>::convert(&i);
    println!("{:?}", f);
}


///
/// 交叉impl
///
/// 目前泛型特化的完整规则依然处于酝酿之中，功能尚不稳定
///
#[test]
fn _22_01_07_cross_impl() {

    trait Foo {}
    trait B {}
    trait C {}

    // 第一个 impl
    impl<T> Foo for T where T: B {}
    // 第二个 impl
    impl<T> Foo for T where T: C {}
}
