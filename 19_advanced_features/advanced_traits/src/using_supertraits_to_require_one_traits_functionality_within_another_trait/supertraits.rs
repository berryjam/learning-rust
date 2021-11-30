use std::fmt;
use std::fmt::{Display, Formatter};

trait OutlinePrint: fmt::Display {
    // 接口继承另外一个接口
    fn outline_print(&self) { // 默认实现
        let output = self.to_string();
        let len = output.len();
        println!("{}", "*".repeat(len + 4));
        println!("*{}*", " ".repeat(len + 2));
        println!("* {} *", output);
        println!("*{}*", " ".repeat(len + 2));
        println!("{}", "*".repeat(len + 4));
    }
}

struct Point {
    x: i32,
    y: i32,
}

impl OutlinePrint for Point {}

/*
由于Point实现了OutlinePrint，而OutlinePrint继承于其他接口，那么Point需要一并把这些继承接口方法都实现了，否则会产生编译错误
 */
impl Display for Point {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}