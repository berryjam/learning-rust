use std::rc::Rc;

fn str_is_dst() {
    /*
    Rust需要在编译时知道类型所占用的内存空间，显然str能存储不同长度的字符串，因此是动态大小类型，下面的代码无法编译通过
    let s1: str = "Hello there!";
    let s2: str = "How's it going?";
     */
    let s1: &str = "Hello there!"; // 虽然str是动态大小类型，但是&str，是对字符串的引用，只用存储字符串的地址和长度，因此大小是固定的
    let s2: &str = "How's it going?";

    let s3: Box<str>;
    let s4: Rc<str>;
}

pub trait Pilot {
    fn fly(&self);
}

pub struct Human;

impl Pilot for Human {
    fn fly(&self) {
        println!("This is your captain speaking.");
    }
}

/*
trait都是dst，使用当作对象使用时，必须与&dyn、Box、Rc等结合使用
 */
fn trait_is_dst() {
    // &dyn Trait
    // Box<dyn Trait>
    // Rc<dyn Trait>

    // let arr:Box<Pilot> = Box::new(Human{}); 编译错误，不能直接声明定义动态类型，需要与引用或者Box等指针结合使用
    let arr: Box<&dyn Pilot> = Box::new(&Human {});
    let arr: Box<dyn Pilot> = Box::new(Human {});
    let arr: Rc<dyn Pilot> = Rc::new(Human {});
}

// Sized是Rust预先定义的特殊trait
fn generic<T>(t: T) {
    // --snip--
}

// 泛型实际都会有一个预定义trait
fn generic_with_default_sized_trait<T: Sized>(t: T) {
    // --snip--
}

// ?Trait语法只能用于Sized接口
fn generic_with_may_be_sized_type<T: ?Sized>(t: &T) {
    // --snip--
}