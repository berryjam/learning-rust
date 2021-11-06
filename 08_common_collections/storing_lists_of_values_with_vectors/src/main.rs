fn main() {
    let v: Vec<i32> = Vec::new();

    let mut v = vec![1, 2, 3]; // vec!宏实际上也会调用Vec::new进行创建vector

    v.push(5);
    v.push(6);
    v.push(7);
    v.push(8);

    {
        let v = vec![1, 2, 3, 4];

        // do stuff with v
    } // <- v goes out of scope and is freed here

    let v = vec![1, 2, 3, 4, 5];

    let third: &i32 = &v[2];
    println!("The third element is {}", third);

    match v.get(2) {
        Some(third) => println!("The third element is {}", third),
        None => println!("There is no third element."),
    }

    let v = vec![1, 2, 3, 4, 5];


    // let does_not_exist = &v[100]; // rust使用下标访问越界的数组元素时，会导致程序panic
    let does_not_exist = v.get(100); // rust使用get方法访问越界的数组元素时，不会panic，只会返回Option<&i32>::None

    let mut v = vec![1, 2, 3, 4, 5];

    let first = &v[0]; // immutable borrow

    v.push(6); // mutable borrow

    // 下面会报错
    // println!("The first element is: {}", first); // Rust不允许某个元素在同一个作用域内，同时存在mutable和immutable引用

    // Iterating over the Values in a Vector
    let v = vec![100, 32, 57];
    for i in &v {
        println!("{}", i);
    }

    let mut v = vec![100, 32, 57];
    for i in &mut v {
        *i += 50; // 通过解引用操作符，去修改mutable引用
    }

    // Using an Enum to Store Multiple Types，虽然vector只能存放相同类型的元素，但是如果使用Enum作为其元素，就可以存储实际不同类型的元素
    enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }

    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];
}
