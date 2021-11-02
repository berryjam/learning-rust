struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

fn main() {
    let mut user1 = User {
        email: String::from("someone@example.com"),
        username: String::from("someusername123"),
        active: true,
        sign_in_count: 1,
    };

    user1.email = String::from("anotheremail@example.com");

    // Creating Instances From Other Instances With Struct Update Syntax
    // let user2 = User {
    //     active: user1.active,
    //     username: user1.username,
    //     email: String::from("another@example.com"),
    //     sign_in_count: user1.sign_in_count,
    // };

    let user2 = User {
        email: String::from("another@example.com"),
        ..user1 // ..语法，表示user1剩余未显示设置的字段将赋值给user2对应的字段，但是user1的username被move到user2了，所以user1后续没法继续使用
    };

    // Using Tuple Structs without Named Fields to Create Different Types
    struct Color(i32, i32, i32);
    struct Point(i32, i32, i32);

    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);

    // Unit-Like Structs Without Any Fields
    struct AlwaysEqual;

    let subject = AlwaysEqual;

    // struct User {
    //     username: &str,  这里需要定义lifetime，否则会编译出错
    //     email: &str, 这里需要定义lifetime，否则会编译出错
    //     sign_in_count: u64,
    //     active: bool,
    // }
    //
    // let user1 = User {
    //     email: "someone@example.com",
    //     username: "someusername123",
    //     active: true,
    //     sign_in_count: 1,
    // };
}

fn build_user(email: String, username: String) -> User {
    User {
        email: email,
        username: username,
        active: true,
        sign_in_count: 1,
    }
}

// Using the Field Init Shorthand when Variables and Fields Have the Same Name (字段名和参数名相同时，可以省略字段名进行赋值)
fn build_user_in_short(email: String, username: String) -> User {
    User {
        email,
        username,
        active: true,
        sign_in_count: 1,
    }
}