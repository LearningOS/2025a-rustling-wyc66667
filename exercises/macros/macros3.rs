mod macros {
    // 使用#[macro_export]导出宏，使其在模块外可用
    #[macro_export]
    macro_rules! my_macro {
        () => {
            println!("Check out my macro!");
        };
    }
}

fn main() {
    my_macro!();
}
