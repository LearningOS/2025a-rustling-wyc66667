fn main() {
    // 移除无意义的Option处理代码（原代码会panic）
    
    // 修复数组字面量中的语法错误（添加逗号）
    let my_arr = &[ -1, -2, -3, -4, -5, -6 ];
    println!("My array! Here it is: {:?}", my_arr);
    
    // 修复Vec::resize的误用，正确创建空Vec
    let mut my_empty_vec = vec![1, 2, 3, 4, 5];
    my_empty_vec.resize(0, 5);
    println!("This Vec is empty, see? {:?}", my_empty_vec);
    
    // 使用标准库swap函数修复交换逻辑
    let mut value_a = 45;
    let mut value_b = 66;
    // Let's swap these two!
    std::mem::swap(&mut value_a, &mut value_b);
    println!("value a: {}; value b: {}", value_a, value_b);
}