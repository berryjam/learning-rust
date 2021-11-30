extern crate proc_macro; // Rust自带的包，不需要显式引入，编译器提供的api，能够让我们的代码读写控制Rust的代码

use proc_macro::TokenStream;
use quote::quote;
// 将syn包解析的ast转回Rust代码
use syn::{Attribute, AttributeArgs, parse_macro_input}; // 解析将Rust代码解析成ast

#[proc_macro_derive(HelloMacro)]
pub fn hello_macro_derive(input: TokenStream) -> TokenStream { // 当在某种类型上注解[derive(HelloMacro)]时，该函数会被自动调用
    // Construct a representation of Rust code as a syntax tree
    // that we can manipulate
    let ast = syn::parse(input).unwrap();

    // Build the trait implementation
    impl_hello_macro(&ast) // 在impl_hello_macro操作ast，实现了变更Rust代码的能力
}

/*
ast结构类似如下：
DeriveInput {
    // --snip--

    ident: Ident {
        ident: "Pancakes",
        span: #0 bytes(95..103)
    },
    data: Struct(
        DataStruct {
            struct_token: Struct,
            fields: Unit,
            semi_token: Some(
                Semi
            )
        }
    )
}
 */

fn impl_hello_macro(ast: &syn::DeriveInput) -> TokenStream {
    let name = &ast.ident;
    let gen = quote! { // quote宏内定义需要执行的Rust代码
        impl HelloMacro for #name {
            fn hello_macro() {
                println!("Hello, Macro! My name is {}!", stringify!(#name));
            }
        }
    };
    gen.into()
}

/*
metadata是属性过程宏里面提供的值，而input是被标注的body
 */
#[proc_macro_attribute]
pub fn my_attribute_marcro(metadata: TokenStream, input: TokenStream) -> TokenStream {
    let args_string = metadata.to_string();
    let gen = quote! {
        println!("args:{}", args_string);
    };
    input
}

#[proc_macro_attribute]
pub fn show_streams(attr: TokenStream, item: TokenStream) -> TokenStream {
    println!("attr: \"{}\"", attr.to_string());
    println!("item: \"{}\"", item.to_string());
    item
}

/*
函数式宏
 */
#[proc_macro]
pub fn make_answer(_item: TokenStream) -> TokenStream {
    "fn answer() -> u32 { 42 }".parse().unwrap()
}