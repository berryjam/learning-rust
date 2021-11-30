use std::fmt;

/*
通过Tuple Structs方式，语法为Struct newType(type1,type2,...,typen),
把外部类型type1,type2等等包装成local类型，绕开了限制
在local crate这里对外部类型Vec<String>实现了外部接口std::fmt::Display
 */
pub struct Wrapper(pub Vec<String>);

pub struct Wrapper1(Vec<String>, Vec<u32>);

impl fmt::Display for Wrapper {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[{}]", self.0.join(", "))
    }
}