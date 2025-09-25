fn main() {
    let my_fav_fruits = vec!["banana", "custard apple", "avocado", "peach", "raspberry"];
    // Step 1: 通过iter()方法创建向量的不可变迭代器
    let mut my_iterable_fav_fruits = my_fav_fruits.iter();
    
    assert_eq!(my_iterable_fav_fruits.next(), Some(&"banana"));
    // Step 2: 第二个元素是"custard apple"
    assert_eq!(my_iterable_fav_fruits.next(), Some(&"custard apple"));
    assert_eq!(my_iterable_fav_fruits.next(), Some(&"avocado"));
    // Step 3: 第四个元素是"peach"
    assert_eq!(my_iterable_fav_fruits.next(), Some(&"peach"));
    assert_eq!(my_iterable_fav_fruits.next(), Some(&"raspberry"));
    // Step 4: 迭代器耗尽后返回None
    assert_eq!(my_iterable_fav_fruits.next(), None);
}
