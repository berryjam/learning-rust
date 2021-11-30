pub type Kilometers = i32;

// 使用alias的主要作用主要是减少重复代码

fn takes_long_type(f: Box<dyn Fn() + Send + 'static>) {
    // --snip--
}

fn returns_long_type() -> Box<dyn Fn() + Send + 'static> {
    // --snip--
    let f: Box<dyn Fn() + Send + 'static> = Box::new(|| println!("hi"));
    f
}

// alias后，将不用重复Box<dyn Fn() + Send + 'static>
type Thunk = Box<dyn Fn() + Send + 'static>;


fn takes_long_type_with_alias_type(f: Thunk) {
    // --snip--
}

fn returns_long_type_with_alias_type() -> Thunk {
    // --snip--
    let f: Thunk = Box::new(|| println!("hi"));
    f
}

use std::fmt;
use std::io::Error;

type Result<T> = std::result::Result<T, std::io::Error>;

// 由于下面接口的泛型E基本都是Error类型，使用alias type之后能减少重复代码
// pub trait Write {
//     fn write(&mut self, buf: &[u8]) -> Result<usize, Error>;
//     fn flush(&mut self) -> Result<(), Error>;
//
//     fn write_all(&mut self, buf: &[u8]) -> Result<(), Error>;
//     fn write_fmt(&mut self, fmt: fmt::Arguments) -> Result<(), Error>;
// }

pub trait Write {
    fn write(&mut self, buf: &[u8]) -> Result<usize>;
    fn flush(&mut self) -> Result<()>;

    fn write_all(&mut self, buf: &[u8]) -> Result<()>;
    fn write_fmt(&mut self, fmt: fmt::Arguments) -> Result<()>;
}