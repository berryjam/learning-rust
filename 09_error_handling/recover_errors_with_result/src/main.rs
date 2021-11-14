use std::fs::File;
use std::io::ErrorKind;
use std::io::{self, Read};

fn main() {
    let f = File::open("hello.txt");

    // Option和Result都是预定义的系统类型，不需要显示引入，Option返回None、Some枚举类型；而Result返回Ok、Err枚举类型
    // let f = match f {
    //     Ok(file) => file,
    //     Err(error) => panic!("Problem opening the file: {:?}", error),
    // };

    // Matching on Different Errors
    let f = match f {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("Problem creating the file: {:?}", e),
            },
            other_error => {
                panic!("Problem opening the file: {:?}", other_error)
            }
        },
    };

    let f = File::open("hello.txt").unwrap_or_else(|error| {
        if error.kind() == ErrorKind::NotFound {
            File::create("hello.txt").unwrap_or_else(|error| {
                panic!("Problem creating the file: {:?}", error);
            })
        } else {
            panic!("Problem opening the file: {:?}", error);
        }
    });

    // Shortcuts for Panic on Error: unwrap and expect
    // unwrap为Result内置方法，封装了一些基本逻辑，当Result为Ok时返回Ok，返回Err时调用panic宏
    let f = File::open("hello.txt").unwrap();

    let f = File::open("hello.txt").expect("Failed to open hello.txt"); // expect与unwrap类型，不过可以使用自定义error msg
}

// Propagating Errors
fn read_username_from_file() -> Result<String, io::Error> {
    let f = File::open("hello.txt");

    let mut f = match f {
        Ok(file) => file,
        Err(e) => return Err(e),
    };

    let mut s = String::new();

    match f.read_to_string(&mut s) { // 最后不要显式return，因为这已经是函数的最后一个expression，可以作为返回值
        Ok(_) => Ok(s),
        Err(e) => Err(e),
    }
}

// A Shortcut for Propagating Errors: the ? Operator
fn read_username_from_file_shortcut() -> Result<String, io::Error> {
    let mut f = File::open("hello.txt")?; // 放置在Result后的?操作符作用：如果返回的是Ok，那么将返回Ok内部的值并继续执行；否则返回Err如果使用return的话
    let mut s = String::new();
    f.read_to_string(&mut s)?;
    Ok(s)
}

fn read_username_from_file_shortcut_chaintogether() -> Result<String, io::Error> {
    let mut s = String::new();

    File::open("hello.txt")?.read_to_string(&mut s)?;

    Ok(s)
}

use std::fs;
fn read_username_from_file_with_api() -> Result<String, io::Error> {
    fs::read_to_string("hello.txt")
}

// The ? Operator Can Be Used in Functions That Return Result
// fn tests() {
//     let f = File::open("hello.txt")?; 编译错误，?操作符只能用在返回Result类型的函数中
// }

// main函数返回类型只能为()或者Result
// fn main() -> Result<(), Box<dyn Error>> {
//     let f = File::open("hello.txt")?;
//
//     Ok(())
// }