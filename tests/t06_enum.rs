use std::fmt;

// Rust的枚举(enum)类型，跟C语言的枚举有点接近，
// 然而更强大，事实上它是一种代数数据类型(Algebraic Data Type)。
// enum 也是可以 impl 的
enum Direction {
    West,
    North,
    South,
    East,
}

impl fmt::Display for Direction {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Direction::East => write!(f, "East"),
            Direction::West => write!(f, "West"),
            Direction::North => write!(f, "North"),
            Direction::South => write!(f, "South"),
        }
    }
}

// 枚举里面居然能包含一些你需要的，特定的数据信息
enum SpecialPoint {
    Point(i32, i32),
    Special(String),
}

#[test]
fn test_enum() {
    println!("{}", Direction::East);
    println!("{}", Direction::West);
    println!("{}", Direction::South);
    println!("{}", Direction::North);

    let sp1 = &SpecialPoint::Point(0, 0);
    let sp2 = &SpecialPoint::Special("Special".to_string());

    show(sp1);
    show(sp2);
}

fn show(sp: &SpecialPoint) {
    // 枚举类型要想访问其成员，几乎无一例外的要用到模式匹配
    match sp {
        SpecialPoint::Point(x, y) => {
            println!("I'am SpecialPoint(x={}, y={})", x, y);
        }
        SpecialPoint::Special(why) => {
            println!("I'am Special because I am {}", why);
        }
    }
}
