use std::num::Wrapping;

fn main() {
    // 变量small初始化为一个非常小的浮点数
    let mut small = std::f32::EPSILON;
    // 不断循环，让small 越来越近于0，直到最后等于0的状态
    while small > 0.0 {
        small = small / 2.0;
        println!("{} {:?}", small, small.classify());
    }

}
