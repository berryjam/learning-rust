// 引用相当于借用
fn main() {
    let s1 = String::from("hello");

    let len = calculate_length(&s1);

    println!("The length of '{}' is {}.", s1, len);

    let s = String::from("hello");

    changeImmutableRef(&s);

    let mut s = String::from("world");
    changeMutableRef(&mut s); // 1.s定义时需要显示指定mut 2.使用时也需要&mut

    let mut s = String::from("hello");

    let r1 = &mut s;
    // let r2 = &mut s; 编译错误，mutable引用有一个限制：限定scope内只能被借用一次，能在编译时就能避免data race
    //
    // println!("{}, {}", r1, r2);

    let mut s = String::from("hello");

    {
        let r1 = &mut s;
    } // r1 goes out of scope here, so we can make a new reference with no problems.

    let r2 = &mut s;

    let mut s = String::from("hello");

    let r1 = &s; // no problem
    let r2 = &s; // no problem  多个immutable引用没有问题，因为不存在data race问题，但是不能既有mutable引用还有immutable引用
    // let r3 = &mut s; // BIG PROBLEM

    // println!("{}, {}, and {}", r1, r2, r3);
    let mut s = String::from("hello");

    let r1 = &s; // no problem
    let r2 = &s; // no problem
    println!("{} and {}", r1, r2);
    // variables r1 and r2 will not be used after this point

    let r3 = &mut s; // no problem
    println!("{}", r3);

    // Dangling References
    // let reference_to_nothing = dangle();  编译错误

    no_dangle();
}

fn calculate_length(s: &String) -> usize { // 使用对象的引用值，不会产生ownship的变更，所以原对象可以继续使用 s is a reference to a String
    s.len()
} // Here, s goes out of scope. But because it does not have ownership of what
// it refers to, nothing happens.

fn changeImmutableRef(some_string: &String) { // 对象默认immutable
    // some_string.push_str(", world"); 编译错误，immutable引用不能修改
}

fn changeMutableRef(some_string: &mut String) { // Mutable References
    some_string.push_str(", world");
}

// Because s is created inside dangle, when the code of dangle is finished, s will be deallocated.
// But we tried to return a reference to it. That means this reference would be pointing to an invalid String.
// That’s no good! Rust won’t let us do this.
// fn dangle() -> &String { // dangle returns a reference to a String
//     let s = String::from("hello"); // s is a new String
//
//     &s // we return a reference to the String, s
// } // Here, s goes out of scope, and is dropped. Its memory goes away.
// // Danger!

fn no_dangle() -> String {
    let s = String::from("hello");

    s
}
