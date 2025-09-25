// Obtain the number of bytes (not characters) in the given argument.
// 添加AsRef<str> trait约束，允许类型转换为&str
fn byte_counter<T: AsRef<str>>(arg: T) -> usize {
    arg.as_ref().as_bytes().len()
}

// Obtain the number of characters (not bytes) in the given argument.
// 添加AsRef<str> trait约束，允许类型转换为&str
fn char_counter<T: AsRef<str>>(arg: T) -> usize {
    arg.as_ref().chars().count()
}

// Squares a number using as_mut().
// 添加AsMut<i32> trait约束，允许获取i32的可变引用
fn num_sq<T: AsMut<i32>>(arg: &mut T) {
    // 通过as_mut()获取i32的可变引用并平方
    let value = arg.as_mut();
    *value *= *value;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn different_counts() {
        let s = "Café au lait";
        assert_ne!(char_counter(s), byte_counter(s));
    }

    #[test]
    fn same_counts() {
        let s = "Cafe au lait";
        assert_eq!(char_counter(s), byte_counter(s));
    }

    #[test]
    fn different_counts_using_string() {
        let s = String::from("Café au lait");
        assert_ne!(char_counter(s.clone()), byte_counter(s));
    }

    #[test]
    fn same_counts_using_string() {
        let s = String::from("Cafe au lait");
        assert_eq!(char_counter(s.clone()), byte_counter(s));
    }

    #[test]
    fn mult_box() {
        let mut num: Box<i32> = Box::new(3);
        num_sq(&mut num);
        assert_eq!(*num, 9);
    }
}