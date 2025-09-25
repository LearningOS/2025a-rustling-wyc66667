use std::num::ParseIntError;

// 修复main函数返回类型，使其能够传播错误
fn main() -> Result<(), ParseIntError> {
    let mut tokens = 100;
    let pretend_user_input = "8";
    let cost = total_cost(pretend_user_input)?;  // 现在可以正确使用?操作符
    
    if cost > tokens {
        println!("You can't afford that many!");
    } else {
        tokens -= cost;
        println!("You now have {} tokens.", tokens);
    }
    
    Ok(())  // 必须返回Result类型的值
}

// 修复total_cost函数的返回类型，并添加类型标注
pub fn total_cost(item_quantity: &str) -> Result<i32, ParseIntError> {
    let processing_fee = 1;
    let cost_per_item = 5;
    let qty = item_quantity.parse::<i32>()?;  // 添加parse的类型标注
    Ok(qty * cost_per_item + processing_fee)
}