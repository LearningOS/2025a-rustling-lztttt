# Vectors

Vectors are one of the most-used Rust data structures. In other programming
languages, they'd simply be called Arrays, but since Rust operates on a
bit of a lower level, an array in Rust is stored on the stack (meaning it
can't grow or shrink, and the size needs to be known at compile time),
and a Vector is stored in the heap (where these restrictions do not apply).

Vectors are a bit of a later chapter in the book, but we think that they're
useful enough to talk about them a bit earlier. We shall be talking about
the other useful data structure, hash maps, later.
向量（Vectors）是 Rust 中最常用的数据结构之一。在其他编程语言中，它们通常被简单地称为 “数组”（Arrays），但由于 Rust 运行在相对底层的层面，Rust 中的 “数组（Array）” 存储在栈内存中 —— 这意味着数组无法动态扩容或缩容，且其长度必须在编译时就确定；而向量（Vector）则存储在堆内存中，因此不受这些限制约束。
向量的相关内容本属于 Rust 官方教程（“the book”，即《Rust 程序设计语言》）中较靠后的章节，但我们认为它的实用性很高，值得提前介绍。至于另一种实用的数据结构 —— 哈希映射（hash maps），我们会在之后再展开讲解。

## Further information

- [Storing Lists of Values with Vectors](https://doc.rust-lang.org/stable/book/ch08-01-vectors.html)
- [`iter_mut`](https://doc.rust-lang.org/std/primitive.slice.html#method.iter_mut)
- [`map`](https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.map)
