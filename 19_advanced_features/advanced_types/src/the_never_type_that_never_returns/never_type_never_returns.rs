/*
fn bar() -> ! {
    // --snip--
}
 */

fn never_type_that_never_returns_example() {
    let guess = "";
    loop {
        let guess: u32 = match guess.trim().parse() { // guess只能有一种类型
            Ok(num) => num,
            Err(_) => continue, // 这里continue返回的就是!类型
        };
    }

    /*
    loop {
        let guess = match guess.trim().parse() { guess只能是一种类型，因此编译出错
            Ok(_) => 5,
            Err(_) => "hello",
        };
    }
     */
}

/*
impl<T> Option<T> {
    pub fn unwrap(self) -> T {
        match self {
            Some(val) => val,
            panic宏，有一个!类型，因此不会返回任何值
            None => panic!("called `Option::unwrap()` on a `None` value"),
        }
    }
}
 */

// One final expression that has the type ! is a loop:
fn loop_is_also_never_type() {
    /*
    Here, the loop never ends, so ! is the value of the expression.
    However, this wouldn’t be true if we included a break, because the loop would terminate when it got to the break.
     */
    print!("forever ");

    loop {
        print!("and ever ");
    }
}