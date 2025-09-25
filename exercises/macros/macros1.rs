macro_rules! my_macro {
    () => {
        println!("Check out my macro!");
    };
}

fn main() {
    // 宏调用需要添加感叹号(!)
    my_macro!();
}
