// primitive_types4.rs
//
// Get a slice out of Array a where the ??? is so that the test passes.
//从数组 a 中取出一个切片，替换掉 ??? 的位置，使得测试能够通过。
// Execute `rustlings hint primitive_types4` or use the `hint` watch subcommand
// for a hint.


#[test]
fn slice_out_of_array() {
    let a = [1, 2, 3, 4, 5];

    let nice_slice = &a[1..4];

    assert_eq!([2, 3, 4], nice_slice)
}
