/* RefCell能够修改已经被immutable引用过的引用
Box<T>能够在编译时保证引用规则，如果违反相应规则则会编译出错
RefCell<T>能够在运行时保证引用规则，如果违反相应规则则会panic和exit，只能用于单线程

1.Rc<T>允许某个对象拥有多个owner，而Box<T>和RefCell<T>只能有一个owner；
2.Box<T>允许immutable和mutable的引用借用，在编译时会检查借用规则，Rc<T>只允许immutable引用借用，在编译时检查；RefCell<T>允许immutable和mutable引用借用，在运行时检查
3.因为RefCell<T>允许mutable引用借用，在运行时检查，即便RefCell<T>是immutable时，仍然可以在RefCell<T>内部修改引用的值
 */

// fn main() {
//     // Enforcing Borrowing Rules at Runtime with RefCell<T>
//
//     // Interior Mutability: A Mutable Borrow to an Immutable Value
//     // let x = 5;
//     // let y = &mut x; 编译错误，x默认是immutable的，y无法对其进行mutable引用
//
//     // A Use Case for Interior Mutability: Mock Objects
// }

pub trait Messenger {
    fn send(&self, msg: &str);
}

pub struct LimitTracker<'a, T: Messenger> {
    messenger: &'a T,
    value: usize,
    max: usize,
}

impl<'a, T> LimitTracker<'a, T>
    where
        T: Messenger,
{
    pub fn new(messenger: &T, max: usize) -> LimitTracker<T> {
        LimitTracker {
            messenger,
            value: 0,
            max,
        }
    }

    pub fn set_value(&mut self, value: usize) {
        self.value = value;

        let percentage_of_max = self.value as f64 / self.max as f64;

        if percentage_of_max >= 1.0 {
            self.messenger.send("Error: You are over your quota!");
        } else if percentage_of_max >= 0.9 {
            self.messenger.send("Urgent warning: You've used up over 90% of your quota!");
        } else if percentage_of_max >= 0.75 {
            self.messenger.send("Warning: You've used up over 75% of your quota!");
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::RefCell;

    struct MockMessenger {
        // sent_messages: Vec<String>,
        sent_messages: RefCell<Vec<String>>,
    }

    impl MockMessenger {
        fn new() -> MockMessenger {
            MockMessenger {
                sent_messages: RefCell::new(vec![]),
            }
        }
    }

    impl Messenger for MockMessenger {
        // fn send(&self, message: &str) {
        //     self.sent_messages.push(String::from(message)); 编译错误，self是immutable，不允许修改
        // }

        // fn send(&mut self, message: &str) {  编译错误，与接口签名不匹配
        //     self.sent_messages.push(String::from(message));
        // }

        fn send(&self, message: &str) {
            self.sent_messages.borrow_mut().push(String::from(message));

            let mut one_borrow = self.sent_messages.borrow_mut();
            // let mut two_borrow = self.sent_messages.borrow_mut(); 编译出错，不允许有两个及以上的mutable引用

            // one_borrow.push(String::from(message));
            // two_borrow.push(String::from(message));
        }
    }

    #[test]
    fn it_sends_an_over_75_percent_warning_message() {
        let mock_messenger = MockMessenger::new();
        let mut limit_tracker = LimitTracker::new(&mock_messenger, 100);

        limit_tracker.set_value(80);

        // assert_eq!(mock_messenger.sent_messages.len(), 1);
        assert_eq!(mock_messenger.sent_messages.borrow_mut().len(), 1);
    }
}