struct A {
    a: i32,
}

impl A {
    pub fn show(&self) {
        println!("{}", self.a);

        // 不能在一个 &self 的方法里调用一个 &mut ref
        // compile error: cannot borrow immutable borrowed content `*self` as mutable
        // self.add_one();
    }
    pub fn add_two(&mut self) {
        self.add_one();
        self.add_one();

        // 可以在一个 &mut self 的方法里调用一个 &self ref
        self.show();
    }
    pub fn add_one(&mut self) {
        self.a += 1;
    }
}

#[test]
fn test_struct_ref() {
    let mut ast = A { a: 12i32 };
    ast.show();
    ast.add_two();
}
