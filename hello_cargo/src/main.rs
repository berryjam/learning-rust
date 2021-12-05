use std::panic;

fn main() {
    let v:u8 = 10;
    let v = vec![0u8; 64];

    let result = panic::catch_unwind(|| {
        println!("hello!");
    });

    assert!(result.is_ok());

    let result = panic::catch_unwind(|| {
        panic!("oh no!");
    });

    // assert!(result.is_err());l(
    println!("world");
}
