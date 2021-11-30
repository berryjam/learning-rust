/*
闭包本质是一种接口，所以不能作为函数返回值，只能用实现了该接口的具体值
下面会出现编译错误
pub fn returns_closure() -> dyn Fn(i32) -> i32 {
    |x| x + 1
}
 */

pub fn returns_closure() -> Box<dyn Fn(i32) -> i32> {
    Box::new(|x| x + 1)
}