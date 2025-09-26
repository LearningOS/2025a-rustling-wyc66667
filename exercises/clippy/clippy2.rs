// clippy2.rs
// 
// Execute `rustlings hint clippy2` or use the `hint` watch subcommand for a
// hint.

// I AM NOT DONE

fn main() {
    let mut res = 42;
    let option = Some(12);
    
    // 用 if let 替代对 Option 的迭代，更符合 Rust 风格
    if let Some(x) = option {
        res += x;
    }
    
    println!("{}", res);
}