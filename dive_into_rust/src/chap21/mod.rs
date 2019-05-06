//!
//! vector内存分配方式为：
//! - 将header部分放在stack
//! - 将剩余部分放在heap buffer
//! - stack上放有指向heap-buffer的指针
//! - stack上放有缓冲区的容量(capacity)
//! - buffer的分配由vector的类型和长度确定
//! - 当heap的分配的buffer空间不够时，即超出了原先设定的容量，进行再分配，并由旧buffer区将数据拷贝到新buffer
//!
//! array内存分配方式：
//! - 由于数组的大小在编译期已经确定，相比heap来说stack的内存分配要高效很多，因此数组完完全全分配在stack上
//!
//! box是相当于一个智能指针，它把原先放在stack上的对象，变换为一个指针以及指向heap的对象的实现，使用box的好处主要为了避免big memory
//!
//!

///
///
#[test]
fn _21_01_01_vec() {

    let mut v1 = Vec::<i32>::new();
    println!("Start: length {} capacity {}", v1.len(), v1.capacity());

    for i in 1..10 {
        v1.push(i);
        println!("[Pushed {}] length {} capacity {}", i, v1.len(), v1.capacity());
    }

    let mut v2 = Vec::<i32>::with_capacity(1);
    println!("Start: length {} capacity {}", v2.len(), v2.capacity());

    v2.reserve(10);

    for i in 1..10 {
        v2.push(i);
        println!("[Pushed {}] length {} capacity {}", i, v2.len(), v2.capacity());
    }
}

///
/// 迭代器
///
/// 因为Vec事先了IntoIterator trait，标准库中的IntoIterator就是编译期留下来的一个扩展
/// 内置for循环语法的接口。任何自定义类型，只要合理地实现了这个trait，就可以被用在内置的for循环里面。
///
#[test]
fn _21_06_01_iter() {

    let mut x = vec![0_i32, 1, 2, 3, 4, 5];

    for item in x {
        println!("{}", item);
    }

    println!("Removed: ");
    for i in x.drain(1..3) {
        println!("{}", i);
    }

    println!("Left: ");
    for i in x.iter() {
        println!("{}", i);
    }
}