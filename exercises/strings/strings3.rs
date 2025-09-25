fn trim_me(input: &str) -> String {
    // 去除两端空白并转换为String
    input.trim().to_string()
}

fn compose_me(input: &str) -> String {
    // 使用format!宏拼接字符串
    format!("{} world!", input)
}

fn replace_me(input: &str) -> String {
    // 替换子串并返回新String
    input.replace("cars", "balloons")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn trim_a_string() {
        assert_eq!(trim_me("Hello! "), "Hello!");
        assert_eq!(trim_me(" What's up!"), "What's up!");
        assert_eq!(trim_me(" Hola! "), "Hola!");
    }

    #[test]
    fn compose_a_string() {
        assert_eq!(compose_me("Hello"), "Hello world!");
        assert_eq!(compose_me("Goodbye"), "Goodbye world!");
    }

    #[test]
    fn replace_a_string() {
        assert_eq!(replace_me("I think cars are cool"), "I think balloons are cool");
        assert_eq!(replace_me("I love to look at cars"), "I love to look at balloons");
    }
}
