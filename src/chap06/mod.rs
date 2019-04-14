
/// 数组
/// 数组中元素的占用空间大小必须是编译期确定的
/// 数组中所容纳的元素个数必须是编译期确定的
///
/// 如果需要使用变长的容器，可以使用标准库中的Vec/LinkedList等
///
/// 表示方式：`[T;n]`
/// `T`表示元素类型
/// `n`表示元素个数
pub fn _06_01_01_array() {
    // 定长数组
    let xs: [i32; 5] = [1, 2, 3, 4, 5];
    // 所有元素初始化为同样的数据，
    let ys: [i32; 500] = [0; 500];
}