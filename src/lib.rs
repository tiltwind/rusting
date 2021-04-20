//! Rust 对文档的哲学，是不要单独写文档，一是代码本身是文档，二是代码的注释就是文档。
//! Rust 不但可以自动抽取代码中的文档，形成标准形式的文档集合，还可以对文档中的示例代码进行测试。
//! The `rusting` crate provides functions that add numbers to other numbers.
//!
//! # Examples
//!
//! ```
//! assert_eq!(4, rusting::add_two(2));
//! ```

/// This function adds two to its argument.
///
/// # Examples
///
/// ```
/// use rusting::add_two;
///
/// assert_eq!(4, add_two(2));
/// ```
pub mod add;
pub mod binding;
pub mod primitive_types;

pub fn add_two(a: i32) -> i32 {
    a + 2
}

// mod 的上面写上 #[cfg(test)] ，表明这个模块是个测试模块
#[cfg(test)]
mod tests {
    use super::add_two;

    // 加上 #[test] 就标明这是一个测试用的函数
    // assert!(expr)               测试表达式是否为 true 或 false
    // assert_eq!(expr, expr)      测试两个表达式的结果是否相等
    // #[should_panic] 标识会失败的用例
    // #[ignore] 不让它参与测试，但是又不想删除它
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }

    #[test]
    fn test_add_two() {
        assert_eq!(4, add_two(2));
    }
}
