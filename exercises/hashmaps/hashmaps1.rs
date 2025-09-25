use std::collections::HashMap;

// 补充哈希映射的类型参数：键为String类型，值为u32类型
fn fruit_basket() -> HashMap<String, u32> {
    // 初始化空的哈希映射
    let mut basket = HashMap::new();
    
    // 已有的香蕉（2个）
    basket.insert(String::from("banana"), 2);
    
    // 添加苹果（3个）和芒果（1个），满足至少三种水果和总数量≥5的要求
    basket.insert(String::from("apple"), 3);
    basket.insert(String::from("mango"), 1);
    
    basket
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn at_least_three_types_of_fruits() {
        let basket = fruit_basket();
        assert!(basket.len() >= 3);
    }

    #[test]
    fn at_least_five_fruits() {
        let basket = fruit_basket();
        assert!(basket.values().sum::<u32>() >= 5);
    }
}
