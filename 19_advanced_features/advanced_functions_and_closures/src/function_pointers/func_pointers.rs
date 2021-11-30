pub fn add_one(x: i32) -> i32 {
    x + 1
}

/*
fn类型为函数指针，能够接受已经定义的函数作为参数
 */
pub fn do_twice(f: fn(i32) -> i32, arg: i32) -> i32 {
    f(arg) + f(arg)
}

pub fn use_closure() {
    let list_of_numbers = vec![1, 2, 3];
    let list_of_strings: Vec<String> =
        list_of_numbers.iter().map(|i| i.to_string()).collect();
}

// 函数指针默认实现了Fn, FnMut, and FnOnce这3种接口
pub fn use_function_pointer() {
    let list_of_numbers = vec![1, 2, 3];
    let list_of_strings: Vec<String> =
        list_of_numbers.iter().map(ToString::to_string).collect(); // 由于有多个to_string，所以需要full qualified
}