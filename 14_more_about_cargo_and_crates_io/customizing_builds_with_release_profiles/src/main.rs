// 通过Cargo.toml的profile.dev及profile.release设置不同的构建profile
// cargo build使用dev profile，cargo build --release使用release profile
fn main() {
    println!("Hello, world!");
}
