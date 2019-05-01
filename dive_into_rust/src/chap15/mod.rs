//!
//!
//!
//!
//! Non-Lexical-Lifetime，非词法生命周期
//!
//!
//! [introduction](http://smallcultfollowing.com/babysteps/blog/2016/04/27/non-lexical-lifetimes-introduction/)
//!

///
/// To better see the distinction between lifetime and scope, let's consider a simple example. In this example,
/// the vector `data` is borrowed (mutably) and the resulting reference is passed to a function `capitalize`.
/// Since `capitalize` does not return the reference back, the __lifetime__ of this borrow will be confined to
/// just that call. The __scope__ of data, in contrast, is much larger, and corresponds to a suffix of
/// the fn body, stretching from the `let` until end of the enclosing scope.
///
///
#[test]
fn _15_01_01_nll() {

    fn foo() {
        let mut data = vec!['a', 'b', 'c']; // --+ 'scope
        capitalize(&mut data[..]);          //   |
    //  ^~~~~~~~~~~~~~~~~~~~~~~~~ 'lifetime //   |
        data.push('d');                     //   |
        data.push('e');                     //   |
        data.push('f');                     //   |
    } // <---------------------------------------+


    fn capitalize(_data: &mut [char]) {
        // do something
    }
}

///
/// references assigned into a variable
///
/// One common problem case is when a reference is assigned into a variable. Consider this trivial
/// variation of the previous example, where the `&mut data[..]` slice is not passed directly to
/// `capitalize`, but is instead stored into a local variable.
///
#[test]
fn _15_01_01_nll_problem_case_variable_references() {

    fn bar() {
        let mut data = vec!['a', 'b', 'c'];
        let slice = &mut data[..]; // <-+ 'lifetime
        capitalize(slice);         //   |
        data.push('d'); // ERROR!  //   |
        data.push('e'); // ERROR!  //   |
        data.push('f'); // ERROR!  //   |
    } // <------------------------------+

    fn capitalize(_data: &mut [char]) {
        // do something
    }

    // you could resolve the problem by putting `slice` into its own block.
    // since we introduced a new block ,the scope of `slice` is now smaller, and hence the resulting lifetime is smaller.
    fn bar1() {
        let mut data = vec!['a', 'b', 'c'];
        {
            let slice = &mut data[..]; // <-+ 'lifetime
            capitalize(slice);         //   |
        } // <------------------------------+
        data.push('d'); // OK
        data.push('e'); // OK
        data.push('f'); // OK
    }
}

///
/// conditional control flow
///
/// Another common problem case is when references are used in only match arm. This most commonly
/// arises around maps. Consider this function, which, given some `key`, processes the value found in
/// `map[key]` if it exists, or else inserts a default value.
///
#[test]
fn _15_01_01_nll_problem_case_conditional_control_flow() {

    use std::collections::HashMap;
    use std::hash::Hash;

    // This code will not compile today. The reason is tha the `map` is borrowed as part of the
    // call to `get_mut` and that borrow must encompass not only the call to `get_mut`. but also
    // the `Some` branch of the match. The innermost expression that encloses both of these expressions
    // is the match itself (as depicted above), and hence the borrow is considered to extend until
    // the end of the match. Unfortunately, the match encloses not only the `Some` branch, but also
    // the `None` branch, and hence when we go to insert into the map in the `None` branch,
    // we get and error that the `map` is still borrowed.
    fn process_or_default<K:Hash+Eq+Copy,V:Default>(map: &mut HashMap<K,V>,
                                       key: K) {
        match map.get_mut(&key) { // -------------+ 'lifetime
            Some(value) => process(value),     // |
            None => {                          // |
                map.insert(key, V::default()); // |
                //  ^~~~~~ ERROR.              // |
            }                                  // |
        } // <------------------------------------+
    }

    // This particular example is relatively easy to workaround. One can (frequently) move the code
    // for `None` out from the `match` like so:
    fn process_or_default1<K:Hash+Eq+Copy,V:Default>(map: &mut HashMap<K,V>,
                                        key: K) {
        match map.get_mut(&key) { // -------------+ 'lifetime
            Some(value) => {                   // |
                process(value);                // |
                return;                        // |
            }                                  // |
            None => {                          // |
            }                                  // |
        } // <------------------------------------+
        map.insert(key, V::default());
    }

    fn process<V>(_value: V) {
        // do something
    }
}

///
/// conditional control flow across functions
///
///
#[test]
fn _15_01_01_nll_problem_case_conditional_control_flow_across_functions() {

    use std::collections::HashMap;
    use std::hash::Hash;

    // 编译器会认为在调用`get_mut(&key)`的时候，产生了一个指向map的`&mut`型引用
    // 而且它的返回值也包含了一个引用，返回值的生命周期是和参数的生命周期一致的。
    // 这个方法的返回值会一直存在于整个match语句块中，所以编译器判定，针对map的引用也是
    // 一直存在于整个match语句块中。导致后面调用insert方法会发生冲突
    fn get_default<K:Hash+Eq+Copy, V: Default>(map: &mut HashMap<K, V>,
                                  key: K)
                                  -> &mut V {
        match map.get_mut(&key) { // -------------+ 'lifetime
            Some(value) => value,              // |
            None => {                          // |
                map.insert(key, V::default()); // |
                //  ^~~~~~ ERROR               // |
                map.get_mut(&key).unwrap()     // |
            }                                  // |
        }                                      // |
    }                                          // v

    fn caller() {
        let mut map: HashMap<i32, u64> = HashMap::new();
        let key = 0_i32;
        {
            let v = get_default(&mut map, key); // -+ 'lifetime
              // +-- get_default() -----------+ //  |
              // | match map.get_mut(&key) {  | //  |
              // |   Some(value) => value,    | //  |
              // |   None => {                | //  |
              // |     ..                     | //  |
              // |   }                        | //  |
              // +----------------------------+ //  |
            process(v);                         //  |
        } // <--------------------------------------+
    }

    fn process<V>(_value: V) {
        // do something
    }

    // 编译依然失败。
    // 原因在于return语句，`get_mut`时候对`map`的借用传递给了`Some(value)`，
    // 在Some这个分支内存在一个引用，指向map的某个部分，而我们又把value返回了，
    // 这意味着编译器认为，这个借用从match开始一直到退出这个函数都存在
    // 因此后面的insert调用依然发生了冲突
    fn get_default1<K:Hash+Eq+Copy, V: Default>(map: &mut HashMap<K, V>,
                                   key: K)
                                   -> &mut V {
        match map.get_mut(&key) { // -------------+ 'lifetime
            Some(value) => return value,       // |
            None => { }                        // |
        }                                      // |
        map.insert(key, V::default());         // |
        //  ^~~~~~ ERROR (still)                  |
        map.get_mut(&key).unwrap()             // |
    }                                          // v


    // 编译成功
    // 区别在于，`get_mut`发生在一个子语句块中。这种情况下，编译器会认为这个借用
    // 跟if外面的代码没什么关系。通过这种方式，我们终于绕过了borrow checker。
    // 但是，为了绕过编译器的限制，我们付出了一些代价。我们需要执行两次hash查找，
    // 一次在contains方法，一次在`get_mut`方法，因此它有额外的性能开销
    fn get_default2<K:Hash+Eq+Copy, V: Default>(map: &mut HashMap<K, V>,
                                   key: K)
                                   -> &mut V {
        if map.contains_key(&key) {
            // ^~~~~~~~~~~~~~~~~~ 'n
            return match map.get_mut(&key) { // + 'lifetime
                Some(value) => value,        // |
                None => unreachable!()       // |
            };                               // v
        }

        // At this point, `map.get_mut` was never
        // called! (As opposed to having been called,
        // but its result no longer being in use.)
        map.insert(key, V::default()); // OK now.
        map.get_mut(&key).unwrap()
    }

    //It’s worth noting that Rust’s hashmaps include an entry API that one could use to implement
    // this function today. The resulting code is both nicer to read and more efficient even than
    // the original version, since it avoids extra lookups on the “not present” path as well:
    fn get_default3<K:Hash+Eq+Copy, V: Default>(map: &mut HashMap<K, V>,
                                   key: K)
                                   -> &mut V {
        map.entry(key)
            .or_insert_with(|| V::default())
    }

    // 让编译器能更准确地分析借用指针的生命周期，不要简单地与scope相绑定，
    // 更符合用户直观思维模式
    #[feacture(nll)]
    fn get_default4<K:Hash+Eq+Copy, V: Default>(map: &mut HashMap<K, V>,
                                                key: K)
                                                -> &mut V {
     match map.get_mut(&key) {
         Some(value) => process(value),
         None => {
             map.insert(key, V::default())
         }
    }
    }
    {
        // NLL 的原理
        // 由于简单的使用 AST 分析最后使用的位置，会导致问题
        // 新版本的借用检查器将 AST 转化为中间表达形式 MIR(middle-level intermediate representation)，这个数据结构会表述一个控制流图
        {
            // 这个功能只影响静态分析结果，不影响程序的执行情况
            // 不会影响以前能通过编译的代码
            // 依然保证了安全性，只是将以前过于保守的检查规则适当放宽
            // 它依赖的是静态检查规则
            // 它只影响引用类型的生命周期，不影响对象的生命周期
            // 它不会影响 RAII 语义
        }
    }
}