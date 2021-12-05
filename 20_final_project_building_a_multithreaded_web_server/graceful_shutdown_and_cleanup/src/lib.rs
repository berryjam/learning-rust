use std::thread;
// 让多个对象共享ownership，Arc是Atomic Reference Counted缩写
use std::sync::Arc;
// 保证线程安全
use std::sync::Mutex;
use std::sync::mpsc;  // mpsc是multiple producer,single consumer缩写，类似golang的channel，但是是多生产者，只有一个消费者

pub struct ThreadPool {
    workers: Vec<Worker>,
    sender: mpsc::Sender<Message>,
}

type Job = Box<dyn FnOnce() + Send + 'static>; // type xx = yy; xx是yy的别名，一般在yy代码较长的情况下使用别名，更简洁可读

enum Message {
    NewJob(Job),
    Terminate,
}

impl ThreadPool {
    /// Create a new ThreadPool.
    ///
    /// The size is the number of threads in the pool.
    ///
    /// # Panics
    ///
    /// The `new` function will panic if the size is zero.
    pub fn new(size: usize) -> ThreadPool {
        assert!(size > 0);

        let (sender, receiver) = mpsc::channel();

        let receiver = Arc::new(Mutex::new(receiver));

        let mut workers = Vec::with_capacity(size);

        for id in 0..size {
            /*
            编译错误，receiver会被move，另外我们需要在线程间共享receiver，也不适合实现Copy接口进行clone
            因此需要共享相同的receiver，而且receiver会涉及到修改，考虑到并发修改，需要避免并发冲突
            考虑使用Arc指针，让多个worker之间共享receiver的ownership，同时使用mutex保证线程安全
            workers.push(Worker::new(id, receiver)); 这里的receiver是纯mpsc::channel返回的receiver
             */
            workers.push(Worker::new(id, Arc::clone(&receiver)));
        }

        ThreadPool { workers, sender }
    }

    /*
    pub fn spawn<F, T>(f: F) -> JoinHandle<T> // std::thread::spawn的实现
    where
        F: FnOnce() -> T,
        F: Send + 'static,
        T: Send + 'static,
     */

    pub fn execute<F>(&self, f: F)
        where
            F: FnOnce() + Send + 'static, // 由于线程每次只会执行闭包一次，Send接口能让thread间传递闭包，另外不确定thread执行花费的时间，所以使用static生命周期确保总是有效
    {
        let job = Box::new(f);

        self.sender.send(Message::NewJob(job)).unwrap();
    }
}

impl Drop for ThreadPool {
    fn drop(&mut self) {
        println!("Sending terminate message to all workers.");

        /*
        这里需要两个循环的原因：如果同一个循环里在发送Terminate之后立马join，不能保证当前循环的worker就是收到消息的那个线程
        因为我们无法保证recv的顺序，是先到先得。假设有两个worker，如果第一个在发送Terminate后，还在忙于处理job，没有recv到message。
        这时，第二个worker接收到message后，第一个worker就一直不会接收到Terminate，也就一直无限loop，而外层一直等待第一个worker结束返回，就会导致死锁
         */
        for _ in &self.workers {
            self.sender.send(Message::Terminate).unwrap();
        }

        println!("Shutting down all workers.");

        for worker in &mut self.workers {
            println!("Shutting down worker {}", worker.id);

            // worker.thread.join().unwrap(); 编译错误，worker是mutable引用，但是join会修改ownership，使用option封装，调用其take方法可以解决，因为take方法能够move
            if let Some(thread) = worker.thread.take() {
                thread.join().unwrap();
            }
        }
    }
}

struct Worker {
    id: usize,
    thread: Option<thread::JoinHandle<()>>, // 由于我们的例子中，闭包不返回任何值，参考spawn的实现，可以返回JoinHandler<()>
}

// struct Job;

impl Worker {
    fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Message>>>) -> Worker {
        let thread = thread::spawn(move || loop { // move能够强制线程内对使用到的变量进行变更owner ship，下面receiver.lock会修改receiver，所以需要move
            /*
            这里lock之后使用unwrap，是考虑到如果某个线程在unlock之前发生panic，导致锁一直无法释放，其他的线程尝试lock就会panic并终止掉，避免产生死锁
            recv之后的unwrap是为了考虑到拥有sending channel的线程挂掉的时候，recevier获取失败导致程序出错，需要终止掉该线程
             */
            let message = receiver.lock().unwrap().recv().unwrap();

            match message {
                Message::NewJob(job) => {
                    println!("Worker {} got a job; executing.", id);

                    job();
                }
                Message::Terminate => {
                    println!("Worker {} was told to terminate.", id);

                    break;
                }
            }
        });

        /*
        let thread = thread::spawn(move || {
            这里while let与let job = xxx的差异在于，let job的右值在赋值之后就会结束，此时会自动调用unlock（没有直接对用户提供）释放掉锁
            但是while let会让receiver一直持有锁，直到while let的block结束后，receiver才会释放锁，因此会产生阻塞，实际运行效率会很差
            while let Ok(job) = receiver.lock().unwrap().recv() {
                println!("Worker {} got a job; executing.", id);

                job();
            }
        });
         */

        Worker { id, thread: Some(thread) }
    }
}