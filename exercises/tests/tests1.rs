// tests1.rs
//
// Tests are important to ensure that your code does what you think it should
// do. Tests can be run on this file with the following command: rustlings run
// tests1
//
// This test has a problem with it -- make the test compile! Make the test pass!
// Make the test fail!
//
// Execute `rustlings hint tests1` or use the `hint` watch subcommand for a
// hint.
// 这个练习的核心是理解 Rust 的单元测试机制：
// 测试函数需要用 #[test] 标注；
// 测试通过的条件：函数执行无 panic、无断言失败；
// 测试失败的条件：主动触发 panic 或断言失败。
// 初始代码（隐含）的问题通常是：缺少测试注解、未触发失败 / 通过逻辑，需要分三步完成（编译 → 通过 → 失败）。
// I AM  DONE

#[cfg(test)]
mod tests {
    #[test]
    fn you_can_assert() {
        assert!(true);
    }
}
