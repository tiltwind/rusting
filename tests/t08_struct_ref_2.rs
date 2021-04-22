struct A {
    a: i32,
}

impl A {
    pub fn show(self) {
        println!("{}", self.a);
    }
}

#[test]
fn test_struct_ref_2() {
    let ast = A { a: 12i32 };
    ast.show();

    // 调用一个函数的时候，如果传入的不是一个引用，那么这个参数将被这个函数吃掉，
    // 即其 owner 将被 move 到这个函数的参数上。
    // 同理，impl 中的 self ，如果你写的不是一个引用的话，也会被默认的 move 掉！
    //
    // 可以在struct 上 加上 #[derive(Copy, Clone)] 解决move的问题, 但这么写实际上也是有其缺陷的
    // 其缺陷就是： Copy 或者 Clone ，都会带来一定的运行时开销！
    //
    // note: this function takes ownership of the receiver `self`, which moves `ast`
    // 5  |     pub fn show(self) {
    //    |                 ^^^^
    // 29 |     println!("{}", ast.a);
    //    |                    ^^^^^ value borrowed here after move

    // println!("{}", ast.a);
}
