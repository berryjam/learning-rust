// Specifying Placeholder Types in Trait Definitions with Associated Types
pub trait IteratorTrait {
    /*
    关联类型，相当于接口的占位符，实现不需要先知道是什么类型，待接口具体实现时替换即可
    所以为什么不直接使用泛型？后面的例子介绍了两者的差异
     */
    type Item;

    fn next(&mut self) -> Option<Self::Item>;
}

/*
 使用泛型的话，像Iterator<String>、Iterator<u32>等各种类型的实现都需要标注具体的类型
*/
pub trait Iterator<T> {
    fn nextGeneric(&mut self) -> Option<T>;
}

impl Iterator<String> for Counter {
    fn nextGeneric(&mut self) -> Option<String> {
        Some("".to_string())
    }
}

impl Iterator<u32> for Counter {
    fn nextGeneric(&mut self) -> Option<u32> {
        Some(0)
    }
}

pub struct Counter {
    pub cnt: u32,
}


impl Counter {
    pub fn new() -> Counter {
        Counter {
            cnt: 0,
        }
    }
}

impl IteratorTrait for Counter {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        let cur_cnt = self.cnt;
        self.cnt = self.cnt + 1;
        if cur_cnt < 5 {
            return Some(cur_cnt);
        }
        return None;
    }
}