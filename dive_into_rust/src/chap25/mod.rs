//!
//!
//! |        容器           |          描述                                |
//! |:---------------------:|:-------------------------------------------:|
//! | `Vec`                 |  可变长数组，连续存储                          |
//! | `VecDeque`            |  双向队列，适用于从头部和尾部插入删除数据         |
//! | `LinkedList`          |  双向链表，非连续存储                          |
//! | `HashMap`             |  基于`Hash`算法存储一系列键值对                 |
//! | `BTreeMap`            |  基于`B`树存储一系列键值对                      |
//! | `HashSet`             |  基于`Hash`算法的集合，相当于没有值的`HashMap`   |
//! | `BTreeSet`            |  基于`B`树的集合，相当于没有值的`BTreeMap`       |
//! | `BinaryHeap`          |  基于二叉堆实现的优先级队列                      |
//!
//!

use std::collections::vec_deque::VecDeque;

///
/// 一个Vec中能存储的元素个数最多为`std::usize::MAX`个，超过了会发生panic。因为它记录元素个数，
/// 用的就是usize类型》如果我们指定元素的类型是0大小的类型，那么，这个Vec根本不需要在堆上分配任何空间。
///
///
#[test]
fn _25_01_01_collections() {

    // 常见的几种构造Vec的方式
    // 1. `new()` 方法与 `default()` 方法一样，构造一个空的`Vec`
    let v1 = Vec::<i32>::default();
    // 2. `with_capacity()`方法可以预先分配一个较大空间，避免插入数据的时候动态扩容
    let v2: Vec<String> = Vec::with_capacity(1000);
    // 3. 利用宏来初始化，语法跟数组初始化类似
    let v3 = vec![1, 2, 3];

    // 插入数据
    let mut v4 = Vec::new();
    // 多种插入数据的方式
    v4.push(1);
    v4.extend_from_slice(&[10, 20, 30, 40, 50]);
    v4.insert(2, 100);
    println!("capacity: {} length: {}", v4.capacity(), v4.len());

    // 访问数据
    // 调用 IndexMut 运算符，可以写入数据
    v4[5] = 5;
    let i = v4[5];
    println!("{}", i);

    // Index 运算符直接访问，如果越界则会造成panic，而get方法不会，因为它返回一个`Option<T>`
    if let Some(i) = v4.get(6) {
        println!("{}", i);
    }

    // Index 运算符支持使用各种 Range 作为索引
    let slice = &v4[4..];
    println!("{:?}", slice);
}

/// 另外，因为Vec里面存在一个指向堆上的指针，它永远是非空的状态，编译器可以据此做优化，使得
/// `size_of::<Option<Vec<T>>>() == size_of::<Vec<T>>()`。
#[test]
fn _23_01_02_collections() {
    struct ZeroSized {}

    let mut v = Vec::<ZeroSized>::new();
    println!("capacity: {} length: {}", v.capacity(), v.len());

    v.push(ZeroSized {});
    v.push(ZeroSized {});
    println!("capacity: {} length: {}", v.capacity(), v.len());

    // p 永远指向 `align_of::<ZeroSized>()`，不需要调用 allocator
    let p = v.as_ptr();
    println!("ptr:{:p}", p);

    let size1 = std::mem::size_of::<Vec<i32>>();
    let size2 = std::mem::size_of::<Option<Vec<i32>>>();
    println!("size of Vec: {} size of option vec: {}", size1, size2);
}


///
/// `VecDeque`
/// VecDeque是一个双向队列。在它的头部或者尾部执行添加或者删除操作，都是效率很高的。它的用法和Vec非常相似，
/// 主要是多了`pop_front()` `push_front()`等方法。
///
#[test]
fn _23_02_01_collections() {
    use std::collections::VecDeque;

    let mut queue = VecDeque::with_capacity(64);
    // 向尾部书序插入一堆数据
    for i in 1..10 {
        queue.push_back(i);
    }
    // 从头部按顺序一个个取出来
    while let Some(i) = queue.pop_front() {
        println!("{}", i);
    }
}

///
/// `HashMap`
/// Rust中的HashMap要求，key要满足Eq+Hash的约束，
///
/// HashMap的查找、插入、删除操作的平均时间复杂度都是O(1)。
///
#[test]
fn _23_03_01_collections() {
    use std::collections::HashMap;

    #[derive(Hash, Eq, PartialEq, Debug)]
    struct Person {
        first_name: String,
        last_name: String,
    }

    impl Person {
        fn new(first: &str, last: &str) -> Self {
            Person {
                first_name: first.to_string(),
                last_name: last.to_string(),
            }
        }
    }

    let mut book = HashMap::new();
    book.insert(Person::new("John", "Smith"), "521-8976");
    book.insert(Person::new("Sandra", "Dee"), "521-9655");
    book.insert(Person::new("Ted", "Baker"), "418-4165");

    let p = Person::new("John", "Smith");

    // 查找键对应的值
    if let Some(phone) = book.get(&p) {
        pritnln!("Phone number found: {}", phone);
    }

    // 删除
    book.remove(&p);

    // 查询是否存在
    println!("Find key: {}", book.contains_key(&p));
}





























