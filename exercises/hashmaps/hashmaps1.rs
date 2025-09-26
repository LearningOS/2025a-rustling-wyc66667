// hashmaps1.rs
//
// A basket of fruits in the form of a hash map needs to be defined. The key
// represents the name of the fruit and the value represents how many of that
// particular fruit is in the basket. You have to put at least three different
// types of fruits (e.g apple, banana, mango) in the basket and the total count
// of all the fruits should be at least five.
//
// Make me compile and pass the tests!
//
// Execute `rustlings hint hashmaps1` or use the `hint` watch subcommand for a
// hint.

// I AM NOT DONE

use std::collections::HashMap;

fn fruit_basket() -> HashMap<String, u32> {
    let mut basket = HashMap::new(); // 初始化一个空的哈希映射

    // 已经提供了两个香蕉
    basket.insert(String::from("banana"), 2);

    // 添加更多水果
    basket.insert(String::from("apple"), 1);
    basket.insert(String::from("mango"), 3);
    // 现在总共有 2 + 1 + 3 = 6 个水果，满足至少5个的要求
    // 水果种类有 banana, apple, mango 三种，满足至少3种的要求

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