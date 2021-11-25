// mpsc是multiple producer,single consumer缩写
use std::sync::mpsc;
use std::thread;
use std::time::Duration;

fn main() {
    let (tx, rx) = mpsc::channel();

    // Channels and Ownership Transference
    thread::spawn(move || {
        let val = String::from("hi");
        tx.send(val).unwrap();
        // println!("val is {}", val); // 编译错误，已经发送的对象，ownership已经被修改，后续就无法访问，除非实现了Copy接口，会进行clone
    });

    // recv会阻塞当前线程直到收到消息，类似golang的channel，当channel关闭会返回错误
    // try_recv不会阻塞线程，通常在一个循环体内使用，尝试获取消息，当没有的时候可以转去处理其他事情，然后回头继续检测是否就绪
    let received = rx.recv().unwrap();
    println!("Got: {}", received);

    // Sending Multiple Values and Seeing the Receiver Waiting
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        let vals = vec![
            String::from("hi"),
            String::from("from"),
            String::from("the"),
            String::from("thread"),
        ];

        for val in vals {
            tx.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    for received in rx { // 类似golang的range channel
        println!("Got: {}", received);
    }

    // Creating Multiple Producers by Cloning the Transmitter
    let (tx, rx) = mpsc::channel();

    let tx1 = tx.clone(); // clone后就能同时发送
    thread::spawn(move || {
        let vals = vec![
            String::from("hi"),
            String::from("from"),
            String::from("the"),
            String::from("thread"),
        ];

        for val in vals {
            tx1.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    thread::spawn(move || {
        let vals = vec![
            String::from("more"),
            String::from("messages"),
            String::from("for"),
            String::from("you"),
        ];

        for val in vals {
            tx.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    for received in rx {
        println!("Got: {}", received);
    }
}
