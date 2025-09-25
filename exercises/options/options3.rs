struct Point {
    x: i32,
    y: i32,
}

fn main() {
    let y: Option<Point> = Some(Point { x: 100, y: 200 });
    
    // 使用ref关键字获取引用，避免所有权移动
    match y {
        Some(ref p) => println!("Co-ordinates are {},{} ", p.x, p.y),
        _ => panic!("no match!"),
    }
    
    y; // 现在可以正常使用y，因为所有权未被移动
}
