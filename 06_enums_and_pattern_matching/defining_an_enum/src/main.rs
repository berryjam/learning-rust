enum IpAddrKind {
    V4,
    V6,
}

struct IpAddr {
    kind: IpAddrKind,
    address: String,
}

enum IpAddrEnum {
    V4(String),
    V6(String),
}

enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

/*
使用enum，比单独定义4个结构体，在使用上更方便，因为可以当作同一种类型使用
struct QuitMessage; // unit struct
struct MoveMessage {
    x: i32,
    y: i32,
}
struct WriteMessage(String); // tuple struct
struct ChangeColorMessage(i32, i32, i32); // tuple struct
*/

impl Message {
    fn call(&self) {
        // method body would be defined here
    }
}

fn main() {
    let home = IpAddr {
        kind: IpAddrKind::V4,
        address: String::from("127.0.0.1"),
    };

    let loopback = IpAddr {
        kind: IpAddrKind::V6,
        address: String::from("::1"),
    };

    // more concise way
    let home = IpAddrEnum::V4(String::from("127.0.0.1"));

    let loopback = IpAddrEnum::V6(String::from("::1"));

    let m = Message::Write(String::from("hello"));
    m.call();

    /* 标准库自带的枚举类型
    enum Option<T> {
        None,
        Some(T),
    }
     */
    let some_number = Some(5); // 隐式声明 Some都会携带数据，不需要显示声明类型
    let some_string = Some("a string");

    let absent_number: Option<i32> = None; //显示声明 Option::None，可以省略Option命名空间，但是由于None不携带类型，所以要显示声明
}
