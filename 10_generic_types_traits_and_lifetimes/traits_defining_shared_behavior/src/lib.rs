/*
A trait tells the Rust compiler about functionality a particular type has and can share with other types.
We can use traits to define shared behavior in an abstract way. We can use trait bounds to specify that a generic type can be any type that has certain behavior.

Note: Traits are similar to a feature often called interfaces in other languages, although with some differences.
trait类似其他语言的interface,表示某种类型具有特定的一些行为
*/

use std::fmt::{Debug, Display};

// trait定义，一个trait可以有多个method
// 限制：不能在外部类型上实现外部trait，
// 比如当前crate为aggregator,Summary也定义在aggregator
// 1.那么可以在aggregator内定义的类型如Tweet上实现Summary
// 2.也可以在aggregator外部的类型如Vec<T>上实现Summary
// 但是不能在外部类型如Vec<T>上实现如Display等外部trait
// 这种设计是为了保证连贯性
pub trait Summary {
    // 可以有默认实现，之后可以被具体的实现覆盖，也可以只有声明fn summarize(&self) -> String;
    fn summarize_author(&self) -> String {
        String::from("(Read more...)")
    }

    fn summarize(&self) -> String {
        format!("(Read more from {}...)", self.summarize_author())
    }
}

pub struct DefaultSummaryImpl {}

impl Summary for DefaultSummaryImpl {} // 如果没有重写方法时，则为默认实现，将使用默认的方法

/**
let article = DefaultSummaryImpl {
};

println!("New article available! {}", article.summarize());
输出New article available! (Read more...)
 */

// Implementing a Trait on a Type
pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

impl Summary for NewsArticle {
    fn summarize(&self) -> String {
        format!("{}, by {} ({})", self.headline, self.author, self.location)
    }
}

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

impl Summary for Tweet {
    fn summarize_author(&self) -> String {
        format!("@{}", self.username)
    }

    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
}

fn testSummaryTrait() {
    let tweet = Tweet {
        username: String::from("horse_ebooks"),
        content: String::from(
            "of course, as you probably already know, people",
        ),
        reply: false,
        retweet: false,
    };

    println!("1 new tweet: {}", tweet.summarize());
}

// Traits as Parameters
pub fn notify(item: &impl Summary) { // 这里Summary可以看作接口类型，item传入不同的实现就实现了多态
    println!("Breaking news! {}", item.summarize());
}

// Trait Bound Syntax
pub fn notify_with_generic<T: Summary>(item: &T) { // 实际上impl trait为这种泛型的语法糖
    println!("Breaking news! {}", item.summarize());
}

// Specifying Multiple Trait Bounds with the + Syntax ，使用+号表示同时具有Summary、Display等trait
pub fn notify_with_multiple_trait(item: &(impl Summary + Display)) {}

pub fn notify_with_multiple_trait_using_generic<T: Summary + Display>(item: &T) {}

// Clearer Trait Bounds with where Clauses 使用where语法能让代码更整洁
fn some_function<T: Display + Clone, U: Clone + Debug>(t: &T, u: &U) -> i32 {
    // ...
    return 1;
}

fn some_function_with_where_clauses<T, U>(t: &T, u: &U) -> i32
    where T: Display + Clone,
          U: Clone + Debug
{
    // ...
    return 1;
}

// Returning Types that Implement Traits
fn returns_summarizable() -> impl Summary {
    Tweet {
        username: String::from("horse_ebooks"),
        content: String::from(
            "of course, as you probably already know, people",
        ),
        reply: false,
        retweet: false,
    }
}

/* 下面试图返回不同类型的Summary类型的代码会发生编译错误
fn returns_summarizable(switch: bool) -> impl Summary {
    if switch {
        NewsArticle {
            headline: String::from(
                "Penguins win the Stanley Cup Championship!",
            ),
            location: String::from("Pittsburgh, PA, USA"),
            author: String::from("Iceburgh"),
            content: String::from(
                "The Pittsburgh Penguins once again are the best \
                 hockey team in the NHL.",
            ),
        }
    } else {
        Tweet {
            username: String::from("horse_ebooks"),
            content: String::from(
                "of course, as you probably already know, people",
            ),
            reply: false,
            retweet: false,
        }
    }
}
 */

fn largest<T: PartialOrd + Copy>(list: &[T]) -> T {
    let mut largest = list[0];

    for &item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}

fn testLargest() {
    let number_list = vec![34, 50, 25, 100, 65];

    let result = largest(&number_list);
    println!("The largest number is {}", result);

    let char_list = vec!['y', 'm', 'a', 'q'];

    let result = largest(&char_list);
    println!("The largest char is {}", result);
}

// Using Trait Bounds to Conditionally Implement Methods
struct Pair<T> {
    x: T,
    y: T,
}

impl<T> Pair<T> {
    fn new(x: T, y: T) -> Self {
        Self { x, y }
    }
}

impl<T: Display + PartialOrd> Pair<T> {
    fn cmp_display(&self) {
        if self.x >= self.y {
            println!("The largest member is x = {}", self.x);
        } else {
            println!("The largest member is y = {}", self.y);
        }
    }
}