fn main() {
    let mut res = 42;
    let option = Some(12);
    
    // 使用if let模式匹配处理Option，替代for循环遍历
    if let Some(x) = option {
        res += x;
    }
    
    println!("{}", res);
}