// macros3.rs
//
// Make me compile, without taking the macro out of the module!
//
// Execute `rustlings hint macros3` or use the `hint` watch subcommand for a
// hint.

// I AM NOT DONE

mod macros {
    macro_rules! my_macro {
        () => {
            println!("Check out my macro!");
        };
    }

    // 将宏导出，使其可以在模块外部使用
    pub(crate) use my_macro;
}

// 引入模块中的宏
use macros::my_macro;

fn main() {
    my_macro!();
}
