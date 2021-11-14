/*
Rust的单元测试默认是并行执行的，如果用例间有依赖关系，需要按顺序执行的话，可以使用--tests-threads参数，更多参数详见--help
cargo tests -- --tests-threads=1
 */
fn prints_and_returns_10(a: i32) -> i32 {
    println!("I got the value {}", a);
    10
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn this_test_will_pass() {
        let value = prints_and_returns_10(4); // 在成功运行的单元测试中，函数内的println并不会有输出，可以使用--show-output进行输出
        assert_eq!(10, value);
    }

    #[test]
    fn this_test_will_fail() {
        let value = prints_and_returns_10(8); // 失败的单元测试中，函数内的println会输出
        assert_eq!(5, value);
    }

    // cargo tests add 将运行名字包含add的测试用例
    #[test]
    fn add_two_and_two() {
        assert_eq!(4, add_two(2));
    }

    #[test]
    fn add_three_and_two() {
        assert_eq!(5, add_two(3));
    }

    // cargo tests one_hundred 限定运行名字包含one_hundred的测试用例
    #[test]
    fn one_hundred() {
        assert_eq!(102, add_two(100));
    }

    #[test]
    #[ignore] // 添加ignore注解，将忽略该测试用例 cargo tests -- --ignored,使用ignored参数，将只运行有ignore注解的测试用例
    fn expensive_test() {
        // code that takes an hour to run
    }
}

pub fn add_two(a: i32) -> i32 {
    a + 2
}