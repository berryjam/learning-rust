/**
Rust的单元测试：单元测试也是属于lib的代码，所以代码也是放置于src目录下面
Rust的集成测试：本身是lib外的代码，只是调用lib中的public方法，所以代码需要放置在src同级目录的test目录中
 */
pub fn add_two(a: i32) -> i32 {
    internal_adder(a, 2)
}

fn internal_adder(a: i32, b: i32) -> i32 { // Rust默认成员对象都是私有的
    a + b
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn internal() { // Rust支持运行私有函数的单元测试
        assert_eq!(4, internal_adder(2, 2));
    }
}