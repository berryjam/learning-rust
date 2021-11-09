fn main() {
    // Cases in Which You Have More Information Than the Compiler
    use std::net::IpAddr;

    let home: IpAddr = "127.0.0.1".parse().unwrap();

    // 如果程序陷入严重状况时，建议使用panic。如果发生的错误可预测，建议用Result。
}

pub struct Guess {
    value: i32,
}

impl Guess {
    pub fn new(value: i32) -> Guess {
        if value < 1 || value > 100 {
            panic!("Guess value must be between 1 and 100, got {}.", value);
        }

        Guess { value }
    }

    pub fn value(&self) -> i32 {
        self.value
    }
}