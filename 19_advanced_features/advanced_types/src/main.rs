mod using_the_newtype_pattern_for_type_safety_and_abstraction;
mod creating_type_synonyms_with_type_aliases;
mod the_never_type_that_never_returns;
mod dynamically_sized_types_and_the_sized_trait;

pub use creating_type_synonyms_with_type_aliases::aliases::Kilometers;

fn main() {
    let x: i32 = 5;
    let y: Kilometers = 5; // Kilometers是i32的alias type，并不是一种新类型，所以仍然可以当作原来的类型来使用

    println!("x + y = {}", x + y);


}
