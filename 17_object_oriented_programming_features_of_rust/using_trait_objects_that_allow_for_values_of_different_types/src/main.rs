use using_trait_objects_that_allow_for_values_of_different_types::{Draw, Button, Screen};

struct SelectBox {
    width: u32,
    height: u32,
    options: Vec<String>,
}

impl Draw for SelectBox {
    fn draw(&self) {
        // code to actually draw a select box
    }
}

fn main() {
    // Trait Objects Perform Dynamic Dispatch,trait使用动态dispatch，而泛型使用静态dispatch，后者性能更好，前者更灵活
    let screen = Screen {
        components: vec![
            Box::new(SelectBox {
                width: 75,
                height: 10,
                options: vec![
                    String::from("Yes"),
                    String::from("Maybe"),
                    String::from("No"),
                ],
            }),
            Box::new(Button {
                width: 50,
                height: 10,
                label: String::from("OK"),
            }),
        ],
    };

    /*
    let screen = Screen {
        components: vec![Box::new(String::from("Hi"))], // 编译错误，String没有实现Draw接口
    };
     */

    screen.run();
}