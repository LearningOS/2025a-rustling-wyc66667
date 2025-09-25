#[cfg(test)]
mod tests {
    #[test]
    fn simple_option() {
        let target = "rustlings";
        let optional_target = Some(target);

        // 使用if let匹配Some模式并提取值
        if let Some(word) = optional_target {
            assert_eq!(word, target);
        }
    }

    #[test]
    fn layered_option() {
        let range = 10;
        let mut optional_integers: Vec<Option<i32>> = vec![None];
        for i in 1..(range + 1) {
            optional_integers.push(Some(i));
        }

        let mut cursor = range;
        // 使用while let匹配嵌套的Some模式（vector.pop()返回Option<Option<i32>>）
        while let Some(Some(integer)) = optional_integers.pop() {
            assert_eq!(integer, cursor);
            cursor -= 1;
        }

        assert_eq!(cursor, 0);
    }
}
