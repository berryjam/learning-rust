/*
Cargo is designed so you can extend it with new subcommands without having to modify Cargo.
If a binary in your $PATH is named cargo-something, you can run it as if it was a Cargo subcommand by running cargo something.
Custom commands like this are also listed when you run cargo --list.
Being able to use cargo install to install extensions and then run them just like the built-in Cargo tools is a super convenient benefit of Cargo’s design!
 */
fn main() {
    println!("Hello, world!");
}
