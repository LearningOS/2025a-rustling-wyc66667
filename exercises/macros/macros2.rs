// 将宏定义移至main函数之前，确保宏在使用前被定义
macro_rules! my_macro {
    () => {
        println!("Check out my macro!");
    };
}

// 添加main函数作为程序入口点
fn main() {
    my_macro!(); // 调用宏，消除未使用警告
}
