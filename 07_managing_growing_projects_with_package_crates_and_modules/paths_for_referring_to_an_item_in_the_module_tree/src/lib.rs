mod front_of_house {
    // module里面的元素默认都是private的，除非声明为public，否则子module可以访问祖先module的元素，但是祖先module不能访问子module的元素
    pub mod hosting {
        // modules声明为pub，只是能够让祖先module能够访问到，但是不表示这个module就是pub的，外部module依旧无法直接访问内部非pub元素
        pub fn add_to_waitlist() {}
    }
}

/*
1.虽然front_of_house不是public的，但是与eat_at_restaurant是同级，在树上是兄弟节点，所以可以访问到front_of_house
2.由于hosting被声明为public，而eat_at_restaurant可以访问到其祖先module front_of_house，所以也可以访问到hosting module
3.同2一样的逻辑，add_to_waitlist被声明为public，eat_at_restaurant可以访问到其祖先module hosting，所以也可以访问到函数add_to_waitlist
*/
pub fn eat_at_restaurant() {
    // Absolute path
    crate::front_of_house::hosting::add_to_waitlist();

    // Relative path
    front_of_house::hosting::add_to_waitlist();

    // Order a breakfast in the summer with Rye toast
    let mut meal = back_of_house::Breakfast::summer("Rye");
    // Change our mind about what bread we'd like
    meal.toast = String::from("Wheat");
    println!("I'd like {} toast please", meal.toast);

    // The next line won't compile if we uncomment it; we're not allowed
    // to see or modify the seasonal fruit that comes with the meal
    // meal.seasonal_fruit = String::from("blueberries");

    let order1 = back_of_house::Appetizer::Soup;
    let order2 = back_of_house::Appetizer::Salad;
}

fn serve_order() {}

mod back_of_house {
    fn fix_incorrect_order() {
        cook_order();
        super::serve_order();
    }

    fn cook_order() {}

    pub struct Breakfast {
        // 尽管struct被声明为public，但是字段默认还是private，除非声明为public
        pub toast: String,
        seasonal_fruit: String,
    }

    impl Breakfast {
        pub fn summer(toast: &str) -> Breakfast {
            Breakfast {
                toast: String::from(toast),
                seasonal_fruit: String::from("peaches"),
            }
        }
    }

    pub enum Appetizer {
        Soup, // enum类型的枚举值默认是public的
        Salad,
    }
}
