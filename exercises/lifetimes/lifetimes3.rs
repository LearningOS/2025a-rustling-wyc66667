// 为结构体添加生命周期参数以明确引用的生命周期
struct Book<'a> {
    author: &'a str,
    title: &'a str,
}

fn main() {
    let name = String::from("Jill Smith");
    let title = String::from("Fish Flying");
    // 创建Book实例，其生命周期受限于name和title的生命周期
    let book = Book {
        author: &name,
        title: &title,
    };
    println!("{} by {}", book.title, book.author);
}
