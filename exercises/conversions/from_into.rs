#[derive(Debug)]
struct Person {
    name: String,
    age: usize,
}

// 实现Default trait作为回退
impl Default for Person {
    fn default() -> Person {
        Person {
            name: String::from("John"),
            age: 30,
        }
    }
}

// 实现From<&str> trait，用于从字符串创建Person
impl From<&str> for Person {
    fn from(s: &str) -> Person {
        // 步骤1: 如果字符串长度为0，返回默认Person
        if s.is_empty() {
            return Person::default();
        }

        // 步骤2: 按逗号分割字符串
        let parts: Vec<&str> = s.split(',').collect();

        // 检查分割后的部分数量是否正好为2
        if parts.len() != 2 {
            return Person::default();
        }

        // 步骤3: 提取第一个元素作为name并去除空白
        let name = parts[0].trim();

        // 步骤4: 如果name为空，返回默认Person
        if name.is_empty() {
            return Person::default();
        }

        // 步骤5: 提取第二个元素并解析为usize作为age
        let age_str = parts[1].trim();
        let age = match age_str.parse() {
            Ok(num) => num,
            // 如果解析失败，返回默认Person
            Err(_) => return Person::default(),
        };

        // 返回实例化的Person对象
        Person {
            name: name.to_string(),
            age,
        }
    }
}

fn main() {
    // 使用`from`函数
    let p1 = Person::from("Mark,20");
    // 由于实现了From，我们也可以使用Into
    let p2: Person = "Gerald,70".into();

    println!("{:?}", p1);
    println!("{:?}", p2);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default() {
        // 测试默认Person是30岁的John
        let dp = Person::default();
        assert_eq!(dp.name, "John");
        assert_eq!(dp.age, 30);
    }

    #[test]
    fn test_bad_convert() {
        // 测试空字符串返回John
        let p = Person::from("");
        assert_eq!(p.name, "John");
        assert_eq!(p.age, 30);
    }

    #[test]
    fn test_good_convert() {
        // 测试"Mark,20"正常转换
        let p = Person::from("Mark,20");
        assert_eq!(p.name, "Mark");
        assert_eq!(p.age, 20);
    }

    #[test]
    fn test_bad_age() {
        // 测试年龄解析失败时返回默认Person
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