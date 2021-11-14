use test_organization; // 与单元测试不同，这里需要显式引入被测试的crate

mod common;

/**
如果不是lib而是binary的话，单元测试无法通过use的方式引入被测试代码，因为binary是意味着自身运行的代码
 */
#[test] // 这里不需要添加#[cfg(tests)]，Rust会在运行cargo test的时候自动编译运行放置在tests目录的test注解代码
fn it_adds_two() {
    common::setup(); // 将一些测试公共函数放置在某个mod下面，可以避免Rust把common也当作集成测试，从而输出一些无关的测试信息
    assert_eq!(4, test_organization::add_two(2));
}