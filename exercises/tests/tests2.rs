// tests2.rs
//
// This test has a problem with it -- make the test compile! Make the test pass!
// Make the test fail!
//
// Execute `rustlings hint tests2` or use the `hint` watch subcommand for a
// hint.

// I AM NOT DONE

#[cfg(test)]
mod tests {
    #[test]
    fn you_can_assert_eq() {
        // 让测试通过：传递两个相等的值
        assert_eq!(2 + 2, 4);
        
        // 让测试失败：传递两个不相等的值
        // assert_eq!(2 + 2, 5);
    }
}
