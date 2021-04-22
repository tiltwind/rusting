// tuple (元组) 表示一个大小、类型固定的有序数据组
#[test]
fn test_tuple() {
    let y = (2, "hello world");
    let x: (i32, &str) = (3, "world hello");

    // 用 let 表达式
    // w=2, z="hello world"
    let (w, z) = y;
    println!("{},{}", w, z);

    // 用下标
    let f = x.0; // f = 3
    let e = x.1; // e = "world hello"
    println!("{},{}", f, e);
}
