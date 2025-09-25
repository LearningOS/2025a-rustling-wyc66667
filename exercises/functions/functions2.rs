fn main() {
    call_me(3);
}

// 将参数类型从i32改为usize，匹配Rust范围语法要求
fn call_me(num: usize) {
    for i in 0..num {
        println!("Ring! Call number {}", i + 1);
    }
}
