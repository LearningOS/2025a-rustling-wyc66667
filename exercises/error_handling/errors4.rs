#[derive(PartialEq, Debug)]
struct PositiveNonzeroInteger(u64);

#[derive(PartialEq, Debug)]
enum CreationError {
    Negative,
    Zero,
}

impl PositiveNonzeroInteger {
    fn new(value: i64) -> Result<Self, CreationError> {
        // 首先检查输入是否为负数
        if value < 0 {
            return Err(CreationError::Negative);
        }
        // 然后检查输入是否为零
        if value == 0 {
            return Err(CreationError::Zero);
        }
        // 只有正数才进行转换并返回Ok
        Ok(PositiveNonzeroInteger(value as u64))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_creation() {
        // 测试正数情况
        assert!(PositiveNonzeroInteger::new(10).is_ok());
        // 测试负数情况
        assert_eq!(
            Err(CreationError::Negative),
            PositiveNonzeroInteger::new(-10)
        );
        // 测试零的情况
        assert_eq!(Err(CreationError::Zero), PositiveNonzeroInteger::new(0));
    }
}
