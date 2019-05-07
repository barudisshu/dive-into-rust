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
use std::collections::btree_map::BTreeMap;

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
        println!("Phone number found: {}", phone);
    }

    // 删除
    book.remove(&p);

    // 查询是否存在
    println!("Find key: {}", book.contains_key(&p));
}

///
///
/// HashMap里面，key存储的位置跟它本身的值密切相关，如果key本身变了，那么它存放的位置
/// 也需要相应变化。所以，HashMap设计的各种API中，指向key的借用一般是只读借用，防止用户
/// 修改它。但是，只读借用并不能完全保证它不被修改，读者应该能想到，只读借用依然可以改变具备
/// 内部可变性特点的类型。
///
#[test]
fn _23_03_02_collections() {
    use std::hash::{Hash, Hasher};
    use std::collections::HashMap;
    use std::cell::Cell;

    #[derive(Eq, PartialEq)]
    struct BadKey {
        value: Cell<i32>,
    }

    impl BadKey {
        fn new(v: i32) -> Self {
            BadKey { value: Cell::new(v) }
        }
    }

    impl Hash for BadKey {
        fn hash<H: Hasher>(&self, state: &mut H) {
            self.value.get().hash(state);
        }
    }

    let mut map = HashMap::new();
    map.insert(BadKey::new(1), 100);
    map.insert(BadKey::new(2), 200);

    for key in map.keys() {
        key.value.set(key.value.get() * 2);
    }

    println!("Find key 1:{:?}", map.get(&BadKey::new(1)));
    println!("Find key 2:{:?}", map.get(&BadKey::new(2)));
    println!("Find key 4:{:?}", map.get(&BadKey::new(4)));

    // 这里设计了一个具备内部可变性的类型作为key。然后直接在容器内部把它的值改变，接下来继续做查找
    // 可以看到，我们再也找不到这几个key了，不论是用修改前的key值，还是用修改后的key值，都找不到。这属于逻辑错误
}

///
/// `BTreeMap`
///
/// BTreeMap对key的要求是满足Ord约束，即具备“全序”特征。
///
///
#[test]
fn _23_03_03_collections() {
    use std::collections::BTreeMap;

    #[derive(Ord, PartialOrd, PartialEq, Eq, Debug, Default)]
    struct Person{
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

    let mut book = BTreeMap::new();
    book.insert(Person::new("John", "Smith"), "521-8976");
    book.insert(Person::new("Sandra", "Dee"), "521-9655");
    book.insert(Person::new("Ted", "Baker"), "418-4165");

    let p = Person::new("John", "Smith");

    // 查找键对应的值
    if let Some(phone) = book.get(&p) {
        println!("Phone number found: {}", phone);
    }

    // 删除
    book.remove(&p);

    // 查询是否存在
    println!("Find key: {}", book.contains_key(&p));
}

///
/// BTreeMap比HashMap多的一项功能是，它不仅可以查询单个key的结果，还可以查询一个区间的结果
///
#[test]
fn _25_03_04_collections() {
    use std::collections::BTreeMap;

    let mut map = BTreeMap::new();
    map.insert(3, "a");
    map.insert(5, "b");
    map.insert(8, "c");

    for (k, v) in map.range(2..6) {
        println!("{} : {}", k, v);
    }
}

///
/// 迭代器
///
///
#[test]
fn _25_04_01_collections() {
    use std::iter::Iterator;

    struct Seq {
        current: i32,
    }

    impl Seq {
        fn new() -> Self {
            Seq { current: 0 }
        }
    }

    impl Iterator for Seq {
        type Item = i32;
        fn next(&mut self) -> Option<i32> {
            if self.current < 100 {
                self.current += 1;
                return Some(self.current);
            } else {
                return None;
            }
        }
    }

    let mut seq = Seq::new();
    while let Some(i) = seq.next() {
        println!("{}", i);
    }
}

///
/// Rust迭代器的强大之处在于可以组合，组合的形式由：
///
/// producer + adapter + consumer
///
///
#[test]
fn _25_04_02_collections() {
    let v = vec![1, 2, 3, 4, 5, 6, 7, 8, 9];
    let mut iter = v.iter()
        .take(5)
        .filter(|&x| x % 2 == 0)
        .map(|&x| x * x)
        .enumerate();
    while let Some((i, v)) = iter.next() {
        println!("{} {}", i, v);
    }
}


///
/// for循环 ，它实际上是对IntoIterator trait的语法糖
///
#[test]
fn _25_05_01_collections() {
    use std::collections::HashMap;
    let v = vec![1, 2, 3, 4, 5, 6, 7, 8, 9];
    for i in v {
        println!("{}", i);
    }
    let map: HashMap<i32, char> = [].iter().cloned().collect();
    for (k, v) in &map {
        println!("{} : {}", k, v);
    }
}


