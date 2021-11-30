/*
有1种声明式的宏：类似C语言的宏

有3种类型的过程宏：类似Java通过反射实现的AOP
1.自定义#[derive]宏，只能用于structs和enums
2.Attribute-like宏，能用于任何对象，定义自定义attribute
3.Function-like宏，像函数一样
 */
mod the_difference_between_macros_and_functions;
mod declarative_macros_with_macro_rules_for_general_metaprogramming;
mod procedural_macros_for_generating_code_from_attributes;
mod how_to_write_a_custom_derive_macro;

/*
不使用过程宏的时候，要使某种类型具有某个接口特性的时候，需要为这个类型实现指定接口才能使用其接口
这样的话，每一种类型都需要为其实现接口
struct Pancakes;

impl HelloMacro for Pancakes {
    fn hello_macro() {
        println!("Hello, Macro! My name is Pancakes!");
    }
}

fn main() {
    Pancakes::hello_macro();
}
 */


// derive过程宏的使用
use hello_macro::HelloMacro;
// 定义接口
use hello_macro_derive::HelloMacro; // 定义aop接口的实现

// attribute过程宏的使用
use hello_macro_derive::my_attribute_marcro; // 引入宏
use hello_macro_derive::show_streams;

// function like过程宏
use hello_macro_derive::make_answer;

#[derive(HelloMacro)] // 定义要注入aop实现的结构体，后续不需要为该结构体实现接口，也可以使用其接口
pub struct Pancakes;

#[my_attribute_marcro("hello", "attribute", "macro")] // 使用属性宏
pub fn use_attribute_macro() {
    println!("use_attribute_macro");
}

#[show_streams]
fn invoke1() {}

#[show_streams(bar)]
fn invoke2() {}

#[show_streams(multiple => tokens)]
fn invoke3() {}

#[show_streams { delimiters }]
fn invoke4() {}

pub fn use_function_macro() {
    make_answer!();
    let res = answer();
    println!("res:{}", res);
    assert_eq!(res, 42)
}

fn main() {
    let vector = my_vec![1,2,3];
    for val in vector {
        println!("val:{}", val);
    }

    Pancakes::hello_macro();

    invoke1();

    use_function_macro();
}
