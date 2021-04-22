//! Rust中的字符串实际上是被编码成UTF-8的一个字节数组
//! 简单来说，Rust字符串内部存储的是一个u8数组，但是这个数组是Unicode字符经过UTF-8编码得来的。
//! 因此，可以看成Rust原生就支持Unicode字符集
//!
//! 在代码里写的，所有的用""包裹起来的字符串，都被声明成了一个不可变，静态的字符串
//!

#[test]
fn test_str() {
    // 实际上是将 "Hello" 这个静态变量的引用传递给了x。同时，这里的字符串不可变！
    let x = "Hello";
    println!("{}", x);

    let z = "foo
bar";
    // 字符串也支持转义字符
    let w = "foo\nbar";
    assert_eq!(z, w);

    //在字符串字面量前加上r来避免转义
    let d: &'static str = r"abc \n abc";
    //等价于
    let c: &'static str = "abc \\n abc";
    assert_eq!(d, c);
}

#[test]
fn test_string() {
    // ---------------------------
    // 在堆上声明的字符串String, 它能动态的去增长或者缩减
    let x: &'static str = "hello";

    let mut y: String = x.to_string();
    println!("{}", y);
    y.push_str(", world");
    println!("{}", y);

    // ---------------------------
    let s = "Hello world".to_string();
    // &*是两个符号&和*的组合，按照Rust的运算顺序，先对String进行Deref,也就是*操作。
    // String实现了 impl Deref<Target=str> for String，相当于一个运算符重载，所以你就能通过*获得一个str类型。
    // 单独的str是不能在Rust里直接存在的，因此需要先给他进行&操作取得&str这个结果
    // 将String转换成&str几乎没有任何开销。但是反过来，将&str转换成String是需要在堆上请求内存的，因此，要慎重.
    let ss: &str = &*s;

    println!("{}", ss);

    // ---------------------------
    // 存储在Vec里的一些字节
    let miao = vec![229, 150, 181];
    // 将一个UTF-8编码的字节数组转换成String，这些字节需是合法的UTF-8编码字符串，直接unwrap()
    let meow = String::from_utf8(miao).unwrap();
    assert_eq!("喵", meow);

    // ---------------------------
    // Rust的字符串不支持通过下标访问
    let x = "你好啊世界".to_string();
    // 转换为字节索引访问
    for i in x.as_bytes() {
        print!("{} ", i);
    }
    println!();
    // 转换为UTF8 char数组访问
    for i in x.chars() {
        print!("{}", i);
    }
    assert_eq!(x.chars().nth(2), Some('啊'));
}

#[test]
fn test_slice() {
    // ---------------------------
    // 字符串切片
    // Rust的字符串Slice实际上是切的bytes。如果你切片的位置正好是一个Unicode字符的内部，Rust会发生Runtime的panic。
    let x = "你好啊世界".to_string();
    let len = x.len();
    let y = &x[9..len];
    print!("{}", y);

    assert_eq!("hello world", "hello ".to_owned() + "world");
}
