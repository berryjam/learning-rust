#[cfg(test)]
mod tests {
    use super::*; // 这里是为了将Rectangle等外部所有module都引入进来，这里使用*号表示所有，与java类似

    #[test]
    fn exploration() {
        let result = 2 + 2;
        assert_eq!(result, 4); // 相比assert_ne宏，eq适合明确值正确的情况，而ne适合那种知道明显不正确的情况
        // 要想将assert_eq、assert_ne宏作用于自定义类型上，需要实现：
        // 1.PartialEq trait(接口)，用于判断两个对象是否相等
        // 2.Debug traits(接口)，用于断言失败的时候，能够打印具体的值
    }

    // Adding Custom Failure Messages
    #[test]
    fn greeting_contains_name() {
        let result = greeting("Carol");
        assert!(result.contains("Carol"), // assert宏后面接一个format宏与具体的值，当失败时将输出对应的错误信息
                "Greeting did not contain name, value was `{}`",
                result);
    }

    #[test]
    // 1.should_panic注解，当testcase没有产生panic的时候，测试用例将失败；这是为了防止程序没有正确处理错误
    // 2.当should_panic使用了expected参数时，如果testcase产生panic的时候，err msg没有包含expected参数时，测试用力也会失败，这是为了保证当测试用例失败时，其产生原因准确性
    #[should_panic(expected = "Guess value must be less than or equal to 100")]
    fn greater_than_100() {
        Guess::new(200);
    }

    // Using Result<T, E> in Tests
    #[test]
    fn it_works() -> Result<(), String> { // 错误有两种方式，一种panic、一种非panic（返回Result枚举值，错误发生时返回Err）
        if 2 + 2 == 4 {
            Ok(())
        } else {
            Err(String::from("two plus two does not equal four"))
        }
    }

    #[test]
    fn it_adds_two() {
        assert_eq!(4, add_two(2));
    }

    #[test]
    fn another() {
        panic!("Make this tests fail");
    }

    #[test]
    fn larger_can_hold_smaller() {
        let larger = Rectangle {
            width: 8,
            height: 7,
        };
        let smaller = Rectangle {
            width: 5,
            height: 1,
        };

        assert!(larger.can_hold(&smaller));
    }

    #[test]
    fn smaller_cannot_hold_larger() {
        let larger = Rectangle {
            width: 8,
            height: 7,
        };
        let smaller = Rectangle {
            width: 5,
            height: 1,
        };

        assert!(!smaller.can_hold(&larger));
    }
}

// Checking Results with the assert! Macro
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

pub fn add_two(a: i32) -> i32 {
    a + 2
}

pub fn greeting(name: &str) -> String {
    format!("Hello {}!", name)
}

pub struct Guess {
    value: i32,
}

impl Guess {
    pub fn new(value: i32) -> Guess {
        if value < 1 {
            panic!(
                "Guess value must be greater than or equal to 1, got {}.",
                value
            );
        } else if value > 100 {
            panic!(
                "Guess value must be less than or equal to 100, got {}.",
                value
            );
        }

        Guess { value }
    }
}