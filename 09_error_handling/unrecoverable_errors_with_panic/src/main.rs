use std::fs::File;
use std::io::ErrorKind;

// RUST_BACKTRACE=1 cargo run ，使用RUST_BACKTRACE环境变量，能看到panic时的完整调用栈
fn main() {
    // panic!("crash and burn");
    let v = vec![1, 2, 3];

    v[99];
}