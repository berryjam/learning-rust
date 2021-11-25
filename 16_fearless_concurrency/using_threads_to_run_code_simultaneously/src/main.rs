use std::thread;
use std::time::Duration;

fn main() {
    // Creating a New Thread with spawn
    let handle = thread::spawn(|| {
        for i in 1..10 {
            println!("hi number {} from the spawned thread!", i);
            thread::sleep(Duration::from_millis(1));
        }
    });

    // Waiting for All Threads to Finish Using join Handles
    handle.join().unwrap(); // join会阻塞当前线程

    for i in 1..5 {
        println!("hi number {} from the main thread!", i);
        thread::sleep(Duration::from_millis(1));
    }

    // Using move Closures with Threads
    let v = vec![1, 2, 3];

    // let handle = thread::spawn(|| {
    //     println!("Here's a vector: {:?}", v); // 编译错误，Rust无法确保v在线程内使用的时候，仍然有效
    // });

    // drop(v);

    let handle = thread::spawn(move || { // move能够强制线程内对使用到的变量进行变更owner ship
        println!("Here's a vector: {:?}", v);
    });

    // drop(v); // 由于在线程内部已经完成对v的ownership变更，因此主线程无法再使用v

    handle.join().unwrap();
}
