use crate::UsState::Alabama;

#[derive(Debug)] // so we can inspect the state in a minute
enum UsState {
    Alabama,
    Alaska,
    // --snip--
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState), // enum类型可以包含任何值，当然也可以包含枚举类型
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => { // 可以使用包含多行代码的代码块，最后一行为expression，最终返回
            println!("Lucky penny!");
            1
        }
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => { // match到Quarter时，将内部变量绑定到state，此后便可以使用
            println!("State quarter from {:?}!", state);
            25
        }
    }
}

// Matching with Option<T>  Option作为内置的标准枚举类型，配合match使用，代码会非常简洁
fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}

fn add_fancy_hat() {}

fn remove_fancy_hat() {}

fn move_player(num_spaces: u8) {}

fn reroll() {}

fn main() {
    let coin = Coin::Quarter(UsState::Alaska);
    println!("value_in_cents: {}", value_in_cents(coin));

    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);

    let dice_roll = 9;
    match dice_roll {
        3 => add_fancy_hat(),
        7 => remove_fancy_hat(),
        other => move_player(other), // catch all模式，必须为最后一行，有点类似一般语言的default
    }

    match dice_roll {
        3 => add_fancy_hat(),
        7 => remove_fancy_hat(),
        _ => reroll(), // 相比catch all模式，_表示忽略匹配的值，避免没有使用匹配值的时候产生warning
    }

    match dice_roll {
        3 => add_fancy_hat(),
        7 => remove_fancy_hat(),
        _ => (), // unit value (the empty tuple type we mentioned in “The Tuple Type” section) 空tuple类型，即单元类型
    }
}
