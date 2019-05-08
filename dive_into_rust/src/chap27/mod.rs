//!
//!
//! 标准库API
//!


///
///
/// Rust给我们提供了一个关键字as用于基本类型的转换。
/// 但是除了基本类型之外，还有更多的自定义类型，它们之间也经常需要做类型转换。
/// 为此，Rust标准库给我们提供了一系列的trait来辅助抽象。
///
/// `AsRef`得到另外一个类型的共享引用，
///
/// ```
/// pub trait AsRef<T: ?Sized> {
///     fn as_ref(&self) -> &T;
/// }
/// ```
///
/// `AsMut`得到另外一个类型的可读写引用，
///
/// ```
/// pub trait AsMut<T: ?Sized> {
///     fn as_mut(&mut self) -> &mut T;
/// }
///```
///
/// 标准库中的String，针对了好几个类型实现了AsRef trait，
///
/// ```
/// impl AsRef<str> for String
/// impl AsRef<[u8]> for String
/// impl AsRef<OsStr> for String
/// impl AsRef<Path> for String
/// ```
///
///
#[test]
fn _27_01_01_as() {

    fn iter_bytes<T: AsRef<[u8]>>(arg: T) {
        for i in arg.as_ref() {
            println!("{}", i);
        }
    }

    let s: String = String::from("this is a string");
    let v: Vec<u8> = vec![1, 2, 3];
    let c: &str = "hello";
    // 相当于函数重载。只不过基于泛型实现的重载，一定需要重载的参数满足某种共同的约束
    iter_bytes(s);
    iter_bytes(v);
    iter_bytes(c);
}

///
/// `Borrow/BorrowMut`
///
#[test]
fn _27_01_02_borrow() {

}

///
/// `From/Into`
///
/// `AsRef/Borrow`做的类型转换都是从一种引用&T到另一种引用&U的转换。
/// 而`From/Into`做的则是从任意类型T到U的类型转换：
///
/// 如果存在`U：From<T>`，则实现`T：Into<U>`。
///
#[test]
fn _27_01_03_from_into() {
    let s: &'static str = "hello";
    let str1: String = s.into();
    let str2: String = String::from(s);

    // 由于这几个trait很常用，因此Rust已经将它们加入到prelude中。
    // 在使用的时候我们不需要写`use std::convert::From;`这样的语句了，包括AsRef、AsMut、Into、From、ToOwned等。
}

///
/// `ToOwned`
///
/// ToOwned trait提供的是一种更“泛化”的Clone的功能。Clone一般是从&T类型变量创造一个新的T类型变量，
/// 而ToOwned一般是从一个&T类型变量创造一个新的U类型变量。
///
/// ```
/// pub enum Cow<'a, B>
///     where
///         B: 'a + ToOwned + ?Sized,
/// {
///     Borrowed(&'a B),
///     Owned(<B as ToOwned>::Owned),
/// }
/// ```
///
#[test]
fn _27_01_04_qto_owned() {

}


///
/// 运算符重载
///
#[test]
fn _27_02_01_operator_override() {
    use std::ops::Add;

    #[derive(Copy, Clone, Debug, PartialEq)]
    struct Complex {
        real: i32,
        imaginary: i32,
    }

    impl Add for Complex {
        type Output = Complex;

        fn add(self, other: Complex) -> Complex {
            Complex {
                real: self.real + other.real,
                imaginary: self.imaginary + other.imaginary,
            }
        }
    }
    impl<'a> Add<&'a Complex> for Complex {
        type Output = Complex;

        fn add(self, other: &'a Complex) -> Complex {
            Complex {
                real: self.real + other.real,
                imaginary: self.imaginary + other.imaginary,
            }
        }
    }

    impl Add<i32> for Complex {
        type Output = Complex;

        fn add(self, other: i32) -> Complex {
            Complex {
                real: self.real + other,
                imaginary: self.imaginary,
            }
        }
    }

    let c1 = Complex { real: 1, imaginary: 2 };
    let c2 = Complex { real: 2, imaginary: 4 };
    println!("{:?}", c1 + c2);

}

///
/// Rust标准库实现了I/O处理
///
#[test]
fn _27_03_01_io() {

    use std::path::PathBuf;
    let mut buf = PathBuf::from("/");
    buf.set_file_name("bar");

    if let Some(s) = buf.to_str() {
        println!("{}", s);
    } else {
        println!("invalid path");
    }

}

///
/// 文件路径
///
#[test]
fn _27_03_02_path() {
    use std::io::prelude::*;
    use std::io::BufReader;
    use std::fs::File;

    fn test_read_file() -> Result<(), std::io::Error> {
        let mut path = std::env::temp_dir();
        path.push(".rustup");
        path.push("settings");
        path.set_extension("toml");

        let file = File::open(&path)?;
        let reader = BufReader::new(file);

        for line in reader.lines() {
            println!("Read a line: {}", line?);
        }
        Ok(())
    }
    match test_read_file() {
        Ok(_) => {}

        Err(e) => {
            println!("Error occured: {}", e);
        }
    }
}

///
/// 标准输入输出
///
#[test]
fn _27_03_03_reader_writer() {
    use std::io::prelude::*;
    use std::io::BufReader;

    fn test_stdin() -> Result<(), std::io::Error> {
        let stdin = std::io::stdin();
        let handle = stdin.lock();
        let reader = BufReader::new(handle);

        for line in reader.lines() {
            let line = line?;
            if line.is_empty() {
                return Ok(());
            }
            println!("Read a line: {}", line);
        }

        Ok(())
    }
    match test_stdin() {
        Ok(_) => {}

        Err(e) => {
            println!("Error occured: {}", e);
        }
    }
}

///
/// 进程启动参数
///
/// 在Rust中，进程启动参数是调用独立的函数`std::env::args()`来得到的，或者使用`std::env::args_os()`来得到，
/// 进程返回值也是调用独立函数`std::process::exit()`来指定。
///
#[test]
fn _27_03_04_env() {

    if std::env::args().any(|arg| arg == "-kill") {
        std::process::exit(1);
    }
    for arg in std::env::args() {
        println!("{}", arg);
    }

}

