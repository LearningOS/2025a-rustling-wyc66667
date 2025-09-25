use std::convert::{TryFrom, TryInto};

#[derive(Debug, PartialEq)]
struct Color {
    red: u8,
    green: u8,
    blue: u8,
}

// 错误类型定义
#[derive(Debug, PartialEq)]
enum IntoColorError {
    // 切片长度不正确
    BadLen,
    // 整数转换错误（超出u8范围）
    IntConversion,
}

// 辅助函数：将i16转换为u8，检查范围
fn convert_component(value: i16) -> Result<u8, IntoColorError> {
    if value >= 0 && value <= 255 {
        Ok(value as u8)
    } else {
        Err(IntoColorError::IntConversion)
    }
}

// 元组实现 (i16, i16, i16) -> Color
impl TryFrom<(i16, i16, i16)> for Color {
    type Error = IntoColorError;
    
    fn try_from(tuple: (i16, i16, i16)) -> Result<Self, Self::Error> {
        Ok(Color {
            red: convert_component(tuple.0)?,
            green: convert_component(tuple.1)?,
            blue: convert_component(tuple.2)?,
        })
    }
}

// 数组实现 [i16; 3] -> Color
impl TryFrom<[i16; 3]> for Color {
    type Error = IntoColorError;
    
    fn try_from(arr: [i16; 3]) -> Result<Self, Self::Error> {
        Ok(Color {
            red: convert_component(arr[0])?,
            green: convert_component(arr[1])?,
            blue: convert_component(arr[2])?,
        })
    }
}

// 切片实现 &[i16] -> Color
impl TryFrom<&[i16]> for Color {
    type Error = IntoColorError;
    
    fn try_from(slice: &[i16]) -> Result<Self, Self::Error> {
        // 检查切片长度是否为3
        if slice.len() != 3 {
            return Err(IntoColorError::BadLen);
        }
        
        Ok(Color {
            red: convert_component(slice[0])?,
            green: convert_component(slice[1])?,
            blue: convert_component(slice[2])?,
        })
    }
}

fn main() {
    // 使用try_from函数
    let c1 = Color::try_from((183, 65, 14));
    println!("{:?}", c1);
    
    // 由于实现了TryFrom，我们可以使用TryInto
    let c2: Result<Color, _> = [183, 65, 14].try_into();
    println!("{:?}", c2);
    
    let v = vec![183, 65, 14];
    // 对于切片，我们应该使用try_from函数
    let c3 = Color::try_from(&v[..]);
    println!("{:?}", c3);
    
    // 或者使用TryInto
    let c4: Result<Color, _> = (&v[..]).try_into();
    println!("{:?}", c4);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tuple_out_of_range_positive() {
        assert_eq!(
            Color::try_from((256, 1000, 10000)),
            Err(IntoColorError::IntConversion)
        );
    }

    #[test]
    fn test_tuple_out_of_range_negative() {
        assert_eq!(
            Color::try_from((-1, -10, -256)),
            Err(IntoColorError::IntConversion)
        );
    }

    #[test]
    fn test_tuple_sum() {
        assert_eq!(
            Color::try_from((-1, 255, 255)),
            Err(IntoColorError::IntConversion)
        );
    }

    #[test]
    fn test_tuple_correct() {
        let c: Result<Color, _> = (183, 65, 14).try_into();
        assert!(c.is_ok());
        assert_eq!(
            c.unwrap(),
            Color {
                red: 183,
                green: 65,
                blue: 14
            }
        );
    }

    #[test]
    fn test_array_out_of_range_positive() {
        let c: Result<Color, _> = [1000, 10000, 256].try_into();
        assert_eq!(c, Err(IntoColorError::IntConversion));
    }

    #[test]
    fn test_array_out_of_range_negative() {
        let c: Result<Color, _> = [-10, -256, -1].try_into();
        assert_eq!(c, Err(IntoColorError::IntConversion));
    }

    #[test]
    fn test_array_sum() {
        let c: Result<Color, _> = [-1, 255, 255].try_into();
        assert_eq!(c, Err(IntoColorError::IntConversion));
    }

    #[test]
    fn test_array_correct() {
        let c: Result<Color, _> = [183, 65, 14].try_into();
        assert!(c.is_ok());
        assert_eq!(
            c.unwrap(),
            Color {
                red: 183,
                green: 65,
                blue: 14
            }
        );
    }

    #[test]
    fn test_slice_out_of_range_positive() {
        let arr = [10000, 256, 1000];
        assert_eq!(
            Color::try_from(&arr[..]),
            Err(IntoColorError::IntConversion)
        );
    }

    #[test]
    fn test_slice_out_of_range_negative() {
        let arr = [-256, -1, -10];
        assert_eq!(
            Color::try_from(&arr[..]),
            Err(IntoColorError::IntConversion)
        );
    }

    #[test]
    fn test_slice_sum() {
        let arr = [-1, 255, 255];
        assert_eq!(
            Color::try_from(&arr[..]),
            Err(IntoColorError::IntConversion)
        );
    }

    #[test]
    fn test_slice_correct() {
        let v = vec![183, 65, 14];
        let c: Result<Color, _> = Color::try_from(&v[..]);
        assert!(c.is_ok());
        assert_eq!(
            c.unwrap(),
            Color {
                red: 183,
                green: 65,
                blue: 14
            }
        );
    }

    #[test]
    fn test_slice_excess_length() {
        let v = vec![0, 0, 0, 0];
        assert_eq!(Color::try_from(&v[..]), Err(IntoColorError::BadLen));
    }

    #[test]
    fn test_slice_insufficient_length() {
        let v = vec![0, 0];
        assert_eq!(Color::try_from(&v[..]), Err(IntoColorError::BadLen));
    }
}