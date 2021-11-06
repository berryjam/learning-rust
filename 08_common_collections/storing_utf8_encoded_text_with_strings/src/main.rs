fn main() {
    // Creating a New String
    let mut s = String::new();

    let data = "initial contents";

    let s = data.to_string(); // 任何类型实现了Display trait,都可以使用to_string方法

    // the method also works on a literal directly:
    let s = "initial contents".to_string(); // 等同于 let s = String::from("initial contents");

    // rust的String使用UTF-8编码
    let hello = String::from("السلام عليكم");
    let hello = String::from("Dobrý den");
    let hello = String::from("Hello");
    let hello = String::from("שָׁלוֹם");
    let hello = String::from("नमस्ते");
    let hello = String::from("こんにちは");
    let hello = String::from("안녕하세요");
    let hello = String::from("你好");
    let hello = String::from("Olá");
    let hello = String::from("Здравствуйте");
    let hello = String::from("Hola");

    // Appending to a String with push_str and push
    let mut s = String::from("foo");
    s.push_str("bar");

    let mut s1 = String::from("foo");
    let s2 = "bar";
    s1.push_str(s2);
    println!("s2 is {}", s2); // push_str接收&str，字符串常量类型作为参数，不会更改对象的ownership，后续还可以继续使用

    let mut s = String::from("lo");
    s.push('l'); // push接收单个字符

    // Concatenation with the + Operator or the format! Macro
    let s1 = String::from("Hello, ");
    let s2 = String::from("world!");
    let s3 = s1 + &s2; // note s1 has been moved here and can no longer be used

    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");

    let s = s1 + "-" + &s2 + "-" + &s3;

    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");

    let s = format!("{}-{}-{}", s1, s2, s3); // 使用format宏对字符串进行拼接，代码更简洁可读

    // Indexing into Strings
    let s1 = String::from("hello");
    // let h = s1[0]; // 注意：rust不支持以下标方式访问字符串

    // 原因举例如下：
    let hello = String::from("Hola"); // UTF8编码，这里长度为4
    let hello = String::from("Здравствуйте"); // UTF8编码，这里长度为24

    // Bytes and Scalar Values and Grapheme Clusters! Oh My!


    // Slicing Strings
    let hello = "Здравствуйте";

    let s = &hello[0..4]; // rust虽然不支持单个下标访问字符串，但是支持range访问字符串，这里s为Зд，注意的是range长度为1效果与单个下标访问相同，也会导致编译错误

    // Methods for Iterating Over Strings
    for c in "नमस्ते".chars() { // chars方法返回unicode编码的字符，这里的字符长度是不固定的，有可能大于1个字节
        println!("{}", c);
    }

    for b in "नमस्ते".bytes() { // bytes方法返回字符串经过UTF8编码后的单个字节数组
        println!("{}", b);
    }
}
