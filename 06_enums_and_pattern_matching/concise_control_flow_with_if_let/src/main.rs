fn main() {
    let config_max = Some(3u8);
    match config_max {
        Some(max) => println!("The maximum is configured to be {}", max),
        _ => (),
    }

    let config_max = Some(3u8);
    if let Some(max) = config_max {
        // 相比上面，这里其实不需要关心Option::None类型的情况，直接用if let语法会更简洁。但是如果Some有多种匹配类型的时候，相对于match语法会缺少exhaustive checking
        // 可以理解成if let是只关心match一种模式而忽略其他模式的一种语法糖
        println!("The maximum is configured to be {}", max);
    }

    let mut count = 0;
    let coin = Coin::Quarter(UsState::Alaska);
    // match coin {
    //     Coin::Quarter(state) => println!("State quarter from {:?}!", state),
    //     _ => count += 1,
    // }

    let mut count = 0;
    if let Coin::Quarter(state) = coin { // 使用if let语法糖，同时可以有else分支
        println!("State quarter from {:?}!", state);
    } else {
        count += 1;
    }
}

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