struct CustomSmartPointer {
    data: String,
}

impl Drop for CustomSmartPointer {
    fn drop(&mut self) { // 当对象out of scope的时候，会自动被调用，通常用于清除创建对象时所新建的资源
        println!("Dropping CustomSmartPointer with data `{}`!", self.data);
    }
}

fn main() {
    let c = CustomSmartPointer {
        data: String::from("my stuff"),
    };
    let d = CustomSmartPointer {
        data: String::from("other stuff"),
    };
    println!("CustomSmartPointers created.");

    // Dropping a Value Early with std::mem::drop
    // c.drop(); 不允许直接调用drop方法，因为会自动调用，避免出现double free
    drop(c); // 可以用drop函数，主动调用对象的drop方法
    println!("CustomSmartPointer dropped before the end of main.");
}
