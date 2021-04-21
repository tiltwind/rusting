#[test]
fn bind_value() {
    // 定义为可绑定变量
    let mut a: f64 = 1.0;
    let b = 2.0f32;
    println!("{},{}", a, b);

    //改变 a 的绑定
    a = 2.0;
    println!("{:?}", a);

    //重新绑定为不可变
    let a = a;
    println!("{:?}", a);

    //不能赋值
    //a = 3.0;

    //类型不匹配
    //assert_eq!(a, b);
}
