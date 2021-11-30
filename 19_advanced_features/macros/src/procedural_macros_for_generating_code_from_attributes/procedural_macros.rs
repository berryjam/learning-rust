/*
过程性宏，与函数类似，一共有3种
1.custom derive
2.attribute-like
3.function-like

使用方法大致如下
use proc_macro;

#[some_attribute]
pub fn some_name(input: TokenStream) -> TokenStream {
}
 */