use std::sync::{Arc, Mutex};
use std::thread;
use std::rc::Rc;

fn main() {
    // Using Mutexes to Allow Access to Data from One Thread at a Time
    let m = Mutex::new(5); // Mutex<i32>，实现了Drop和Deref接口，是一个智能指针
    // 所以可以使用解引用操作符*
    // 当out of scope的时候，自动unlock

    {
        // lock返回mutable引用，类型为LockResult，其中Ok时类型为MutexGuard<i32>
        let mut num = m.lock().unwrap();
        *num = 6;
    }

    println!("m = {:?}", m);

    // Sharing a Mutex<T> Between Multiple Threads
    // let counter = Mutex::new(0); // 编译错误，不能重复move
    // let counter = Rc::new(Mutex::new(0)); // 编译错误，不是threads safely，Rc<T>没有实现Send接口，线程不安全
    let counter = Arc::new(Mutex::new(0)); // Arc是Atomic Rc<T>缩写
    let mut handles = vec![];

    for _ in 0..10 {
        let counter = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            let mut num = counter.lock().unwrap(); // 编译错误，对counter进行重复move

            *num += 1;
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("Result: {}", *counter.lock().unwrap());

    // Multiple Ownership with Multiple Threads
}
