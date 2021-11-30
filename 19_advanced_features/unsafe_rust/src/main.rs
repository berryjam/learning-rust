/**
Rust的编译器具有静态分析能力，会误报但不会漏报，所以有点过于保守
引入unsafe的目的，是在用户能够确保程序不会存在问题下，让编译器能通过其代码
但是如果用户代码确实存在问题，将导致memory leak、空指针等问题
 */
fn main() {
    // Unsafe Superpowers
    /* 在unsafe代码块内，能够进行以下不能在safe Rust所进行的操作
    Dereference a raw pointer
    Call an unsafe function or method
    Access or modify a mutable static variable
    Implement an unsafe trait
    Access fields of unions
     */
    dereferencing_a_raw_pointer();
    calling_an_unsafe_function_or_method();
    creating_a_safe_abstraction_over_unsafe_code();
    using_extern_functions_to_call_external_code();
    accessing_or_modifying_a_mutable_static_variable();
    implementing_an_unsafe_trait();
}

// Dereferencing a Raw Pointer 其中主要使用场景是，是用raw指针访问C代码
fn dereferencing_a_raw_pointer() {
    /* 两类raw指针，*const T and *mut T，其中*不是解引用操作符，而是名字的一部分
     其中*const T为immutable，*mut T为mutable，相比引用和智能指针，具有以下特点
    1.Are allowed to ignore the borrowing rules by having both immutable and mutable pointers or multiple mutable pointers to the same location
    2.Aren’t guaranteed to point to valid memory
    3.Are allowed to be null
    4.Don’t implement any automatic cleanup
     */
    let mut num = 5;

    let r1 = &num as *const i32; // 声明定义raw指针，可以不必在unsafe block内，但是使用时必须在其内部
    let r2 = &mut num as *mut i32; // 这里通过引用创建raw指针，可以保证所创建的指针都是有效的

    unsafe { // 必须在unsafe block才能访问raw指针
        println!("r1 is: {}", *r1);
        println!("r2 is: {}", *r2);
    }

    let r1 = &num;
    let r2 = &mut num;

    // println!("r1 is: {}", *r1); 编译错误，不能对同时存在immutable和mutable引用的非immutable值进行访问

    let address = 0x012345usize;
    let r = address as *const i32; // 这里引用内存某个地址创建raw指针，构成未定义行为，不保证其有效
}

// Calling an Unsafe Function or Method
fn calling_an_unsafe_function_or_method() {
    unsafe {
        dangerous();
    }
}

unsafe fn dangerous() { // unsafe函数体内本身就是unsafe block，所以调用其他unsafe函数时不用再声明unsafe block
    another_dangerous();
}

unsafe fn another_dangerous() {}


// Creating a Safe Abstraction over Unsafe Code
fn creating_a_safe_abstraction_over_unsafe_code() {
    let mut v = vec![1, 2, 3, 4, 5, 6];

    let r = &mut v[..];

    let (a, b) = r.split_at_mut(3); // split_at_mut是一个safe函数，但是其内部有unsafe code

    assert_eq!(a, &mut [1, 2, 3]);
    assert_eq!(b, &mut [4, 5, 6]);

    /*
    use std::slice;

    let address = 0x01234usize;
    let r = address as *mut i32;

    let slice: &[i32] = unsafe { slice::from_raw_parts_mut(r, 10000) }; 会产生未定义行为
     */
}

// 这个函数会出现编译错误，只用safe rust无法实现该函数，说明unsafe的必要性
// fn split_at_mut(slice: &mut [i32], mid: usize) -> (&mut [i32], &mut [i32]) {
//     let len = slice.len();
//
//     assert!(mid <= len);
//
//     (&mut slice[..mid], &mut slice[mid..]) 编译器静态分析过于保守，虽然引用同一个slice的不同部分是安全的，但是编译器只认作两个返回值都是对同一个slice引用，因此不允许其编译通过
// }

use std::slice;

fn split_at_mut(slice: &mut [i32], mid: usize) -> (&mut [i32], &mut [i32]) {
    let len = slice.len();
    let ptr = slice.as_mut_ptr();

    assert!(mid <= len);

    unsafe {
        (
            slice::from_raw_parts_mut(ptr, mid),
            slice::from_raw_parts_mut(ptr.add(mid), len - mid),
        )
    }
}

// Using extern Functions to Call External Code
fn using_extern_functions_to_call_external_code() {
    unsafe {
        // 在Rust调用其他语言的接口
        println!("Absolute value of -3 according to C: {}", abs(-3));
    }
}

extern "C" {
    // extern block，列出其他要被Rust调用的函数的名字和方法签名及返回值
    fn abs(input: i32) -> i32;
}

// extern函数，不同于extern block定义的函数，这是定义Rust要被其他语言调用的函数
#[no_mangle]
pub extern "C" fn call_from_c() {
    println!("Just called a Rust function from C!");
}

// Accessing or Modifying a Mutable Static Variable
static HELLO_WORLD: &str = "Hello, world!"; // Rust的全局变量及静态变量,访问immutable静态变量是safe的，但是访问mutable静态变量则是unsafe

fn accessing_or_modifying_a_mutable_static_variable() {
    println!("name is: {}", HELLO_WORLD);

    add_to_count(3);

    unsafe {
        println!("COUNTER: {}", COUNTER);  // 凡是调用mutable静态变量都需要处于unsafe block
    }
}

static mut COUNTER: u32 = 0;

fn add_to_count(inc: u32) {
    unsafe {
        COUNTER += inc; // 凡是调用mutable静态变量都需要处于unsafe block
    }
}

// Implementing an Unsafe Trait
fn implementing_an_unsafe_trait() {
    unsafe trait Foo {
        // methods go here
    }

    unsafe impl Foo for i32 {
        // method implementations go here
    }
}

// Accessing Fields of a Union
/*
The final action that works only with unsafe is accessing fields of a union.
A union is similar to a struct, but only one declared field is used in a particular instance at one time.
Unions are primarily used to interface with unions in C code.
Accessing union fields is unsafe because Rust can’t guarantee the type of the data currently being stored in the union instance.
You can learn more about unions in the reference.
 */

// When to Use Unsafe Code
/*
Using unsafe to take one of the five actions (superpowers) just discussed isn’t wrong or even frowned upon.
But it is trickier to get unsafe code correct because the compiler can’t help uphold memory safety.
When you have a reason to use unsafe code, you can do so, and having the explicit unsafe annotation makes it easier to track down the source of problems when they occur.
 */