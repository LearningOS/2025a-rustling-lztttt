// lifetimes2.rs
//
// So if the compiler is just validating the references passed to the annotated
// parameters and the return type, what do we need to change?
//那么如果编译器只是对传递给带标注参数和返回类型的引用进行有效性校验，我们需要修改什么呢？
// Execute `rustlings hint lifetimes2` or use the `hint` watch subcommand for a
// hint.

// I AM  DONE

fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

fn main() {
    let string1 = String::from("abcd");
    let string2 = "xyz";                
    let result = longest(string1.as_str(), string2);
    println!("The longest string is '{}'", result);
}
