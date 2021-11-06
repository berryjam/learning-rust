mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}
    }
}

// use crate::front_of_house::hosting; // 绝对路径引入，一般use后接元素的父module，而不是直接引用到元素自身，如use crate::front_of_house::hosting::add_to_waitlist; 否则使用的时候会误以为add_to_waitlist为当前module的元素
use self::front_of_house::hosting; // 相对路径引入

pub fn eat_at_restaurant() {
    hosting::add_to_waitlist();
    hosting::add_to_waitlist();
    hosting::add_to_waitlist();
}

use std::collections::HashMap;

pub fn use_hashmap() {
    let mut map = HashMap::new();
    map.insert(1, 2);
}

use std::fmt;

use std::io::Result as IoResult; // 使用别名，提升可读性

fn function1() -> fmt::Result {
    // --snip--
    return fmt::Result::Ok(())
}

fn function2() -> io::Result<()> {
    let e = io::Error::new(io::ErrorKind::Other, "oh no!");
    // --snip--
    return  io::Result::Err(e);
}

fn functions3() -> IoResult<String> {
    // --snip
    return io::Result::Ok(String::from("ok"));
}

use rand::Rng;

// Using Nested Paths to Clean Up Large use Lists
/*
将多条有公共前缀的use合并到一行，更简洁
use std::cmp::Ordering;
use std::io;
 */
// --snip--
// use std::{cmp::Ordering, io};
// --snip--

// 合并有公共前缀的父子module,使用self
use std::io::{self, Write};

use std::collections::*; // 慎用*，这样在使用过程中，不容易区分元素所属的具体module
