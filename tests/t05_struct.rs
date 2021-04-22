// Import (via `use`) the `fmt` module to make it available.
use std::fmt;

// 具名结构体
// 内部每个成员都有自己的名字和类型。
struct A {
    attr1: i32,
    atrr2: String,
}

// 元组类型结构体
// 元组类型结构体使用小括号，类似 tuple
// 可以看作是一个有名字的元组，具体使用方法和一般的元组基本类似。
struct B(i32, u16, bool);

// 空结构体
// 结构体内部也可以没有任何成员
// 空结构体的内存占用为0
struct D {}

// To use the `{}` marker, the trait `fmt::Display` must be implemented
// manually for the type.
impl fmt::Display for D {
    // This trait requires `fmt` with this exact signature.
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Write strictly the first element into the supplied output
        // stream: `f`. Returns `fmt::Result` which indicates whether the
        // operation succeeded or failed. Note that `write!` uses syntax which
        // is very similar to `println!`.
        write!(f, "this is empty struct")
    }
}

struct Person {
    name: String,
}

// 通过impl对一个结构体添加成员方法
impl Person {
    // 构造函数
    fn new(n: &str) -> Person {
        Person {
            name: n.to_string(),
        }
    }

    fn greeting(&self) {
        println!("{} say hello.", self.name);
    }
}

impl fmt::Display for Person {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "my name is {}", self.name)
    }
}

#[test]
fn test_struct() {
    let a = A {
        attr1: 1,
        atrr2: "test".to_string(),
    };
    println!("{},{}", a.attr1, a.atrr2);

    let b = B(123, 12, true);
    println!("{},{},{}", b.0, b.1, b.2);

    let d = D {};
    println!("{}", d);

    let p = Person::new("Wongoo");
    p.greeting();
    println!("{}", p);
}
