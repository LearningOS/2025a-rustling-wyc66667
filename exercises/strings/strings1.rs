fn main() {
    let answer = current_favorite_color();
    println!("My current favorite color is {}", answer);
}

fn current_favorite_color() -> String {
    // 将字符串字面量转换为String类型
    "blue".to_string()
}
