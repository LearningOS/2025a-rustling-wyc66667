// from_into.rs
//
// The From trait is used for value-to-value conversions. If From is implemented
// correctly for a type, the Into trait should work conversely. You can read
// more about it at https://doc.rust-lang.org/std/convert/trait.From.html
//
// Execute `rustlings hint from_into` or use the `hint` watch subcommand for a
// hint.

#[derive(Debug)]
struct Person {
    name: String,
    age: usize,
}

// 实现 Default 特性作为转换失败时的 fallback
impl Default for Person {
    fn default() -> Person {
        Person {
            name: String::from("John"),
            age: 30,
        }
    }
}

// 实现 From<&str> 特性，将字符串转换为 Person 结构体
impl From<&str> for Person {
    fn from(s: &str) -> Person {
        // 步骤1: 如果输入字符串为空，返回默认 Person
        if s.is_empty() {
            return Person::default();
        }

        // 步骤2: 按逗号分割字符串
        let parts: Vec<&str> = s.split(',').collect();

        // 检查是否正好分割出两个部分
        if parts.len() != 2 {
            return Person::default();
        }

        // 步骤3: 提取姓名并处理空白字符
        let name = parts[0].trim();
        
        // 步骤4: 如果姓名为空，返回默认 Person
        if name.is_empty() {
            return Person::default();
        }

        // 步骤5: 提取年龄字符串并处理空白
        let age_str = parts[1].trim();
        
        // 解析年龄为 usize 类型
        let age = match age_str.parse::<usize>() {
            Ok(num) => num,
            Err(_) => return Person::default(),
        };

        // 返回有效的 Person 实例
        Person {
            name: name.to_string(),
            age,
        }
    }
}

fn main() {
    // 使用 from 函数进行转换
    let p1 = Person::from("Mark,20");
    // 由于实现了 From，我们也可以使用 Into 进行转换
    let p2: Person = "Gerald,70".into();
    println!("{:?}", p1);
    println!("{:?}", p2);
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_default() {
        // 测试默认 Person 是 30 岁的 John
        let dp = Person::default();
        assert_eq!(dp.name, "John");
        assert_eq!(dp.age, 30);
    }
    
    #[test]
    fn test_bad_convert() {
        // 测试空字符串返回默认 Person
        let p = Person::from("");
        assert_eq!(p.name, "John");
        assert_eq!(p.age, 30);
    }
    
    #[test]
    fn test_good_convert() {
        // 测试 "Mark,20" 能正确转换
        let p = Person::from("Mark,20");
        assert_eq!(p.name, "Mark");
        assert_eq!(p.age, 20);
    }
    
    #[test]
    fn test_bad_age() {
        // 测试年龄解析失败时返回默认 Person
        let p = Person::from("Mark,twenty");
        assert_eq!(p.name, "John");
        assert_eq!(p.age, 30);
    }

    #[test]
    fn test_missing_comma_and_age() {
        let p: Person = Person::from("Mark");
        assert_eq!(p.name, "John");
        assert_eq!(p.age, 30);
    }

    #[test]
    fn test_missing_age() {
        let p: Person = Person::from("Mark,");
        assert_eq!(p.name, "John");
        assert_eq!(p.age, 30);
    }

    #[test]
    fn test_missing_name() {
        let p: Person = Person::from(",1");
        assert_eq!(p.name, "John");
        assert_eq!(p.age, 30);
    }

    #[test]
    fn test_missing_name_and_age() {
        let p: Person = Person::from(",");
        assert_eq!(p.name, "John");
        assert_eq!(p.age, 30);
    }

    #[test]
    fn test_missing_name_and_invalid_age() {
        let p: Person = Person::from(",one");
        assert_eq!(p.name, "John");
        assert_eq!(p.age, 30);
    }

    #[test]
    fn test_trailing_comma() {
        let p: Person = Person::from("Mike,32,");
        assert_eq!(p.name, "John");
        assert_eq!(p.age, 30);
    }

    #[test]
    fn test_trailing_comma_and_some_string() {
        let p: Person = Person::from("Mike,32,man");
        assert_eq!(p.name, "John");
        assert_eq!(p.age, 30);
    }
}
