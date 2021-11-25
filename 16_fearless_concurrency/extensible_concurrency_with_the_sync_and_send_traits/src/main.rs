/*
Send和Sync接口是Marker traits，这是一种特殊的接口，没有任何方法需要实现
 */
fn main() {
    // Allowing Transference of Ownership Between Threads with Send
    //  two concurrency concepts are embedded in the language: the std::marker traits Sync and Send
    // Send接口，表明对象可以在线程间进行ownership变更,但是Rc<T>不能在多线程环境中使用

    // Allowing Access from Multiple Threads with Sync
    // Sync接口，表明对象在多个线程间进行引用是线程安全的

    // Implementing Send and Sync Manually Is Unsafe
}
