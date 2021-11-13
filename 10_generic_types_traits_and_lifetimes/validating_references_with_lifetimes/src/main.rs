/**
lifetimes可以作为注解作用于引用对象，主要作用是防止dangling引用，只有在函数签名的时候通过lifetime注解标注好各个对象的生命周期，编译器才能知晓各自的生命周期
1.作用在函数参数和返回值引用时，Rust能够把参数和返回值的生命周期关联起来，保证了内存安全
2.作用在struct内部的引用时......

当引用在缺少显式的lifetime注解时，Rust会对引用应用3种规则 （input lifetime:函数或者方法参数的lifetime output lifetime:函数返回值的lifetime）
1.每个参数都有自己的lifetime
2.如果只有一个input lifetime参数时，这个lifetime会应用到所有output lifetime参数
3.如果有多个input lifetime参数时，并且其中有一个是&self或者&mut self时，self的lifetime会应用到所有output lifetime参数
{
        let r;

        {
            let x = 5;
            r = &x;
        }

        println!("r: {}", r); // 这里x已经销毁，但是在上面r有一个对其的引用，如果不阻止其编译，将出现dangling引用
    }
 */
fn main() {
    let string1 = String::from("abcd");
    let string2 = "xyz";

    let result = longest(string1.as_str(), string2);
    println!("The longest string is {}", result);

    let string1 = String::from("long string is long");

    {
        let string2 = String::from("xyz");
        let result = longest(string1.as_str(), string2.as_str());
        println!("The longest string is {}", result);
    }

    /*
    let string1 = String::from("long string is long");
    let result;
    {
        let string2 = String::from("xyz");
        result = longest(string1.as_str(), string2.as_str());
    }
    println!("The longest string is {}", result); // 编译错误
     */

    let novel = String::from("Call me Ishmael. Some years ago...");
    let first_sentence = novel.split('.').next().expect("Could not find a '.'");
    let i = ImportantExcerpt {
        part: first_sentence,
    };
}

/*
// The Borrow Checker
fn borrow_checker() {
    {
        let r;                      // ---------+-- 'a
                                    //          |
        {                           //          |
            let x = 5;              // -+-- 'b  |
            r = &x;                 //  |       |
        }                           // -+       |
                                    //          |
        println!("r: {}", r);       //          |
    }                               // ---------+

    {
        let x = 5;                  // ----------+-- 'b
                                    //           |
        let r = &x;                 // --+-- 'a  |
                                    //   |       |
        println!("r: {}", r);       //   |       |
                                    // --+       |
    }                               // ----------+
}
*/

// Generic Lifetimes in Functions
// fn longest(x: &str, y: &str) -> &str { // 由于x与y的生命周期有可能不一样，Rust为了避免返回值在后续出现dangling引用，会出现编译错误
//     if x.len() > y.len() {
//         x
//     } else {
//         y
//     }
// }

// Lifetime Annotation Syntax 注意：lifetime注解不会改变引用的生命周期
fn lifetime_annotation_syntax<'a>( // 下面表示b和c具有相同的生命周期
                                   a: &i32, // a reference
                                   b: &'a i32, // a reference with an explicit lifetime
                                   c: &'a mut i32, // a mutable reference with an explicit lifetime
) {}

// Lifetime Annotations in Function Signatures
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

// Thinking in Terms of Lifetimes
fn longest_with_lifetimes<'a>(x: &'a str, y: &str) -> &'a str { // 函数参数是否要有lifetime注解，取决于函数逻辑，当函数返回值与参数引用值有关联时，返回值的生命周期必须与该参数一致
    x
}

// 尽管下面有lifetime注解，但是其返回值与该lifetime没有任何关联，实际会产生dangling引用，因此会出现编译错误
// fn longest_with_dangling_ref<'a>(x: &str, y: &str) -> &'a str {
//     let result = String::from("really long string");
//     result.as_str()
// }

// Lifetime Annotations in Struct Definitions
struct ImportantExcerpt<'a> {
    part: &'a str,
}

// Lifetime Elision
fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}

// Rust 1.0的时候，函数返回引用值时，必须严格定义lifetime注解
fn first_word_before_has_lifetime_annotation<'a>(s: &'a str) -> &'a str {
    return s;
}

// Lifetime Annotations in Method Definitions
impl<'a> ImportantExcerpt<'a> {
    fn level(&self) -> i32 {
        3
    }
}

/*
There are two input lifetimes, so Rust applies the first lifetime elision rule and gives both &self and announcement their own lifetimes.
Then, because one of the parameters is &self, the return type gets the lifetime of &self, and all lifetimes have been accounted for.
 */
impl<'a> ImportantExcerpt<'a> {
    fn announce_and_return_part(&self, announcement: &str) -> &str {
        println!("Attention please: {}", announcement);
        self.part
    }
}

// The Static Lifetime
fn static_lifetime() {
    let s: &'static str = "I have a static lifetime."; // s将编译进binary，其生命周期贯穿整个程序，一直都可用
}

// Generic Type Parameters, Trait Bounds, and Lifetimes Together
use std::fmt::Display;

// 综合使用泛型、trait（接口，如果泛型实现了trait,则限定这些类型具有相同的一类行为）、lifetime注解的例子
fn longest_with_an_announcement<'a, T>(
    x: &'a str,
    y: &'a str,
    ann: T,
) -> &'a str
    where
        T: Display,
{
    println!("Announcement! {}", ann);
    if x.len() > y.len() {
        x
    } else {
        y
    }
}