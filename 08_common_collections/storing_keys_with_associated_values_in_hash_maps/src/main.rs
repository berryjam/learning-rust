fn main() {
    // Creating a New Hash Map
    use std::collections::HashMap;

    let mut scores = HashMap::new(); // hash同vector类似，只能存放相同类型的元素

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    let teams = vec![String::from("Blue"), String::from("Yellow")];
    let initial_scores = vec![10, 50];

    let mut scores: HashMap<_, _> =
        teams.into_iter().zip(initial_scores.into_iter()).collect();

    // Hash Maps and Ownership

    let field_name = String::from("Favorite color");
    let field_value = String::from("Blue");

    let mut map = HashMap::new();
    map.insert(field_name, field_value);
    // field_name and field_value are invalid at this point, try using them and
    // see what compiler error you get!

    // 像实现了Copy trait的i32类型，hashmap存储时会复制对应的值，不会修改ownership，后续还可以继续使用。同样地，引用也不会被修改ownership。

    // Accessing Values in a Hash Map
    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    let team_name = String::from("Blue");
    let score = scores.get(&team_name); // hashmap的get方法会返回Option<&V>

    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    // 遍历hashmap
    for (key, value) in &scores { // 这里的immutable引用，不修改ownership，但是要修改key value的时候，需要用mutable引用
        println!("{}: {}", key, value);
    }

    // Updating a Hash Map
    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Blue"), 25);

    println!("{:?}", scores);

    // Only Inserting a Value If the Key Has No Value
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);

    scores.entry(String::from("Yellow")).or_insert(50); // entry(xx).or_insert(xxx) 只会在没有对应的key value的时候插入
    scores.entry(String::from("Blue")).or_insert(50);

    println!("{:?}", scores);

    // Updating a Value Based on the Old Value
    let text = "hello world wonderful world";

    let mut map = HashMap::new();

    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0); // or_insert返回value的mutable引用，所以下面通过解引用可以修改对应的value
        *count += 1;
    }

    println!("{:?}", map);

    // hashmap使用的默认hash函数为SipHash，也可以通过自行实现BuildHasher trait，修改使用的hash函数，会有不同的性能和安全效果
}
