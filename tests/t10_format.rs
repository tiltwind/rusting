// format_string := <text> [ format <text> ] *
// format := '{' [ argument ] [ ':' format_spec ] '}'
// argument := integer | identifier
//
// format_spec := [[fill]align][sign]['#'][0][width]['.' precision][type]
// fill := character
// align := '<' | '^' | '>'
// sign := '+' | '-'
// width := count
// precision := count | '*'
// type := identifier | ''
// count := parameter | integer
// parameter := integer '$'
#[test]
fn test_format_number() {
    println!("{}", format!("{}", 123));   // 123
    // > 是一个语义，它表示的是生成的字符串向右对齐，与之相对的还有<(向左对齐)和^(居中)
    println!("{}", format!("{:>5}", 123)); // __123
    println!("{}", format!("{:>05}", 123)); // 00123
    println!("{}", format!("{:0>5}", 123)); // 00123
    println!("{}", format!("{:<5}", 123)); // 123__
    println!("{}", format!("{:<05}", 123)); // 00123
    println!("{}", format!("{:0<5}", 123)); // 12300

    println!("{}", format!("{:^5}", 12)); // _12__
    println!("{}", format!("{:^05}", 12)); // 00012
    println!("{}", format!("{:0^5}", 12)); // 01200

    println!("{}", format!("{1},{0}", 123, 234)); // 234,123

    // format!宏调用的时候参数可以是任意类型，而且是可以position参数和key-value参数混合使用的。
    // 但是要注意的一点是，key-value的值只能出现在position值之后并且不占position
    println!("{}", format!("{1:>0width$},{0}", 123, 234, width = 6)); // 000234,123
    println!("{}", format!("{1:>0width$},{0:0<width$}, {height}", 123, 234, width = 6, height = 180)); // 000234,123000, 180

    println!("{}", format!("{:>8}", 12.34)); // ___12.34
    println!("{}", format!("{:>08}", 12.34)); // 00012.34
    println!("{}", format!("{:>8}", -12.34)); // __-12.34
    println!("{}", format!("{:>08}", -12.34)); // -0012.34

    println!("{}", format!("{:.2}", 12.34567)); // 12.35
}
