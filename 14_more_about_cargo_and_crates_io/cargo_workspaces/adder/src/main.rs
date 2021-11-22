use add_one;
// use rand; 即使同workspace下的其他package已经添加rand的依赖，但自身还需要依赖

fn main() {
    let num = 10;
    println!(
        "Hello, world! {} plus one is {}!",
        num,
        add_one::add_one(num)
    );
}