mod function_pointers;
mod returning_closures;

pub use crate::function_pointers::func_pointers::{add_one, do_twice};

fn main() {
    let answer = do_twice(add_one, 5);

    println!("The answer is: {}", answer);
}
