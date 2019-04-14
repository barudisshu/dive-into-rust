
/// 数组
/// 数组中元素的占用空间大小必须是编译期确定的
/// 数组中所容纳的元素个数必须是编译期确定的
///
/// 如果需要使用变长的容器，可以使用标准库中的Vec/LinkedList等
///
/// 表示方式：`[T;n]`
/// `T`表示元素类型
/// `n`表示元素个数
#[test]
fn _06_01_01_array() {
    // 定长数组
    let xs: [i32; 5] = [1, 2, 3, 4, 5];
    // 所有元素初始化为同样的数据，
    let ys: [i32; 500] = [0; 500];
}

/// 同类型的数组之间可以相互赋值
#[test]
fn _06_01_02_array() {
    let mut xs: [i32; 5] = [1, 2, 3, 4, 5];
    let ys: [i32; 5] = [6, 7, 8, 9, 10];
    xs = ys;
    println!("new array {:?}", xs);
}

/// 把数组xs作为参数传给一个函数，这个数组并不会退化成一个指针
/// 而是将这个数组完整复制进这个函数。函数体内对数组的改动不会影响到外面的数组
/// Rust支持usize类型的索引的数组，索引从0开始计数
#[test]
fn _06_01_03_array() {
    let v: [i32; 5] = [1, 2, 3, 4, 5];
    let x = v[0] + v[1];    // 把第一个元素和第二个元素的值相加
    println!("sum is {}", x);
}

/// 内置方法
#[test]
fn _06_01_04_array() {

    // 只要包含元素，数组是可比较的
    let v1 = [1, 2, 3];
    let v2 = [1, 2, 4];
    println!("{:?}", v1 < v2);

    // 对数组进行遍历操作
    let v = [0_i32; 10];
    for i in &v {
        println!("{:?}", i);
    }

    // 目前标准库中，数组本身没有实现IntoIterator trait，但是数组切片是实现了，
    // 所以可以直接在for in循环中使用数组切片，而不能直接使用数组本身
}

/// 多维数组
#[test]
fn _06_01_05_array() {

    let v: [[i32; 2]; 3] = [[0,0],[0,0],[0,0]];

    for i in &v {
        println!("{:?}", i);
    }
}

/// 数组切片
/// 对数组借用borrow操作，可以生成一个“数组切片”(Slice)。
/// 数组切片对数组没有“所有权”，我们可以把数组切片看做专门用于指向数组的指针，
/// 是对数组的另外一个“视图”。
#[test]
fn _06_01_06_array() {
    // 比如，有一个数组`[T;n]`，它的借用指针的类型就是`&[T;n]`。
    // 它在编译器内部转换为数组切片类型`&[T]`。
    // 数组切片实质上还是指针，它不过是在类型系统中丢弃了编译阶段定长数组类型的长度信息，
    // 而将长度信息存储为运行期的值。

    fn mut_array(a: &mut [i32]) {
        a[2] = 5;
    }

    println!("size of &[i32; 3] : {:?}", std::mem::size_of::<&[i32; 3]>());
    println!("size of &[i32] : {:?}", std::mem::size_of::<&[i32]>());

    let mut v: [i32; 3] = [1, 2, 3];
    {
        let s: &mut [i32; 3] = &mut v;  // s是个数组切片，它实际上是个胖指针(fat pointer)。
        mut_array(s);
    }

    println!("{:?}", v);
}

/// Range
/// Rust中的Range代表一个“区间”，一个“范围”，它有内置的语法支持，就是两个小数点`..`。
#[test]
fn _06_02_01_range() {
    // r是一个Range<i32>，中间是两个点，代表`[1, 10)`这个区间
    let r = 1..10;
    for i in r {
        print!("{:?}\t", i);
    }
}

/// Range
#[test]
fn _06_02_02_range() {
    use std::ops::Range;
    let r = Range { start: 1, end: 10 };
    for i in r {
        print!("{:?}\t", i);
    }

}

/// 两个小数点的语法仅仅是一个“语法糖”而已，用它构造出来的变量是Range类型
/// 这个类型本身实现了Iterator trait，因此它可以直接应用到循环语句中
/// Range具有迭代器的全部功能，因此它能调用迭代器的成员方法。
#[test]
fn _06_02_03_range() {
    use std::iter::Iterator;
    // 先用rev方法把这个区间反过来，然后用map方法把每个元素乘以10
    let r = (1i32..11).rev().map(|i| i * 10);

    for i in r {
        print!("{:?}\t", i);
    }
}

/// 在Rust中，还有其他几种Range，包括
/// `std::ops::RangeFrom`代表只有起始没有结束的范围，语法为`start..`，含义是`[start, +∞]`
/// `std::ops::RangeTo` 代表没有起始只有结束的范围，语法为`..end`，对有符号数的含义是`(-∞, end)`，对无符号数的含义是`[0, end)`。
/// `std::ops::RangeFull` 代表没有上下界限的范围，语法为`..`，对有符号数的含义是`(-∞, +∞)`，对无符号数的含义是`[0, +∞)`。
/// 数组和Range之间最常用的配合就是使用Range进行索引操作
#[test]
fn _06_02_04_slice() {

    fn print_slice(arr: &[i32]) {
        println!("Length: {}", arr.len());

        for item in arr {
            print!("{}\t", item);
        }
        println!("");
    }

    let arr: [i32; 5] = [1, 2, 3, 4, 5];
    print_slice(&arr[..]);  // full range

    let slice = &arr[2..];
    print_slice(slice);     // RangeFrom

    let slice2 = &slice[..2];
    print_slice(slice2);    // RangeTo

    // Rust还提供了一种左闭右闭区间的语法，它使用这种语法来表示`..=`
    // 闭区间对应的标准库中的类型是：
    // `std::ops::RangeInclusive`，语法为`start..=end`，含义是`[start, end]`
    // `std::ops::RangeToInclusive`，语法为`..=end`，对有符号数的含义是`(-∞, end]`，对无符号数的含义是`[0, end]`
}

/// 边界检查
#[test]
fn _06_03_01_bound() {
    // index超过了数组的真实长度范围，会执行`panic!`操作，导致线程abort
    // 为了防止索引操作导致程序崩溃，如果我们不确定使用的“索引”是否合法，
    // 应该使用`get()`方法调用来获取数组中的元素，这个方法不会引起`panic!`，它的返回类型是`Option<T>`

    let v = [10i32, 20 , 30, 40, 50];
    let first = v.get(0);
    let tenth = v.get(10);
    println!("{:?} {:?}", first, tenth);

}

/// 从效率上看，Rust比C/C++的数组索引效率低一点，因为C/C++的索引操作是不执行任何安全性检查的，它们
/// 对应的Rust代码相当于调用`get_unchecked()`函数。在Rust中，更加地道的做法是尽量使用“迭代器”方法。
#[test]
fn _06_03_02_iterator() {
    use std::iter::Iterator;

    let v = &[10i32, 20, 30, 40, 50];
    // 如果我们同时需要index和内部元素的值，调用`enumerate()`方法
    for (index, value) in v.iter().enumerate() {
        println!("{} {}", index, value);
    }
    // filter方法可以执行过滤，nth函数可以获取第n个元素
    let item = v.iter().filter(|&x| *x % 2 == 0).nth(2);
    println!("{:?}", item);
}

/// 字符串
/// Rust的字符串涉及两种类型，
/// 一种是`&str`
/// 一种是`String`
#[test]
fn _06_04_01_string() {
    // `str`是DST类型
    // `&str`是字符串切片类型
    let greeting: &str = "Hello";
    let substr: &str = &greeting[2..];
    println!("{}", substr);

    // `&str`类型也是一个胖指针，可以用下面的示例证明
    println!("Size of pointer: {}", std::mem::size_of::<*const ()>());
    println!("Size of &str   : {}", std::mem::size_of::<&str>());

    // 它内部实际上包含了一个指向字符串片段的头部的指针和一个长度。
}

/// `String` 类型
/// 它跟&str类型的主要区别是，它有管理内存空间的权力。
/// `&str`类型是对一块字符串区间的借用，它对所指向的内存空间没有所有权，哪怕`&mut str`也一样
#[test]
fn _06_04_02_string() {
    let greeting: &str = "Hello";
    // 我们没有办法扩大greeting所引用的范围，在它后面增加内容。
    // 但`String`类型可以
    let mut s = String::from("Hello");
    s.push(' ');
    s.push_str("World.");
    println!("{}", s);

    // 这是因为String类型在堆上动态申请了一块内存空间，它有权对这块内存空间进行扩容
    // 内部实现类似于`std::Vec<u8>`类型
    // 所以我们可以把这个类型作为容纳字符串的容器使用
    // 这个类型实现了`Deref<Target=str>`的trait。
    // 所以在很多情况下，`&String`类型可以被编译器自动转换为`&str`类型

    fn capitalize(substr: &mut str) {
        substr.make_ascii_uppercase();
    }

    capitalize(&mut s);
    println!("{}", s);

    // 这个例子中，capitalize函数调用的时候，形式参数要求是`&mut str`类型
    // 而实际参数是`&mut String`类型，这里编译器给我们做了自动类型转换
    // 在capitalize函数内部，它有权修改`&mut str`所指向的内容，但是无权给这个字符串扩容或者释放内存
}
































