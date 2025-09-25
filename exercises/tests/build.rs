//! This is the build script for both tests7 and tests8.
//!
//! You should modify this file to make both exercises pass.

fn main() {
    // 为tests7设置TEST_FOO环境变量
    let timestamp = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_secs();
    // 设置环境变量TEST_FOO为当前时间戳
    let your_command = format!("rustc-env=TEST_FOO={}", timestamp);
    println!("cargo:{}", your_command);

    // 为tests8启用"pass"特性
    let your_command = "rustc-cfg=feature=\"pass\"";
    println!("cargo:{}", your_command);
}
