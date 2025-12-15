// primitive_types6.rs
//
// Use a tuple index to access the second element of `numbers`. You can put the
// expression for the second element where ??? is so that the test passes.
//使用元组索引来访问 numbers 的第二个元素。你可以将表示第二个元素的表达式放在 ??? 的位置，以便测试通过。
// Execute `rustlings hint primitive_types6` or use the `hint` watch subcommand
// for a hint.


#[test]
fn indexing_tuple() {
    let numbers = (1, 2, 3);
    // Replace below ??? with the tuple indexing syntax.
    let second = numbers.0;

    assert_eq!(1, second,
        "This is not the 2nd number in the tuple!")
}
