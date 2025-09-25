// 将宏定义移至main函数之前，确保宏在使用前被定义
macro_rules! my_macro {
    () => {
        println!("Check out my macro!");
    };
}
