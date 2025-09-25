//! 为tests7设置必要的环境变量

fn main() {
    // 获取当前Unix时间戳（秒）
    let timestamp = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_secs();
    
    // 向Cargo输出指令，设置TEST_FOO环境变量
    // 格式为：cargo:rustc-env=VAR=VALUE
    println!("cargo:rustc-env=TEST_FOO={}", timestamp);
}
    