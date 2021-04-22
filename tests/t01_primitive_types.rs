#[test]
pub fn primitive_show() {
    // boolean type
    let t = true;
    let f: bool = false;
    println!("t={},f={}", t, f);

    // char type
    let c = 'c';
    println!("c={}", c);

    // numeric types
    let x = 42;
    let y: u32 = 123_456;
    let z: f64 = 1.23e+2;
    let zero = (z - 123.4).abs();
    let bin = 0b1111_0000;
    let oct = 0o7320_1546;
    // the literal `0xf23a_b049` (decimal `4063932489`) does not fit into the type `i32` and will become `-231034807i32`
    // let hex = 0xf23a_b049;
    let hex = 0x723a_b000;

    println!(
        "x={},y={},z={},zero={},bin={},oct={},hex={}",
        x, y, z, zero, bin, oct, hex
    );

    // string types
    let str = "Hello, world!";
    let string = str.to_string();
    println!("str={},string={}", str, string);

    // arrays and slices
    let a = [0, 1, 2, 3, 4];
    let middle = &a[1..4];
    let mut ten_zeros: [i64; 10] = [0; 10];
    ten_zeros[1] = 11;
    println!("middle={},ten_zeros={}", middle[0], ten_zeros[1]);

    // tuples
    let tuple: (i32, &str) = (50, "hello");
    let (fifty, _) = tuple;
    let hello = tuple.1;
    println!("fifty={},hello={}", fifty, hello);

    // raw pointers
    let x = 5;
    let raw = &x as *const i32;
    let points_at = unsafe { *raw };
    println!("points_at={}", points_at);

    // functions
    fn foo(x: i32) -> i32 {
        x
    }
    let bar: fn(i32) -> i32 = foo;
    println!("bar={}", bar(123));
}
