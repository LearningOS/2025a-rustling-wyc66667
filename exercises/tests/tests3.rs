// tests3.rs
//
// This test isn't testing our function -- make it do that in such a way that
// the test passes. Then write a second test that tests whether we get the
// result we expect to get when we call `is_even(5)`.
//
// Execute `rustlings hint tests3` or use the `hint` watch subcommand for a
// hint.

pub fn is_even(num: i32) -> bool {
    num % 2 == 0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_true_when_even() {
        // 测试偶数时应返回true
        assert!(is_even(2));
        assert!(is_even(4));
        assert!(is_even(0)); // 0是偶数
    }

    #[test]
    fn is_false_when_odd() {
        // 测试奇数时应返回false，包括题目要求的is_even(5)
        assert!(!is_even(1));
        assert!(!is_even(3));
        assert!(!is_even(5)); // 测试is_even(5)的情况
    }
}
