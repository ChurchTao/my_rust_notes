fn main() {
    println!("Hello, world!");
    let list = vec![10, 21, 7, -1, 33, 43];
    println!("Hello, world! max = {}", largest(&list));
    println!("Hello, world! max = {}", largest2(&list));
    let list = vec!['c', 'a', 'b', '1', 'l', 'z'];
    println!("Hello, world! max = {}", largest(&list));
    println!("Hello, world! max = {}", largest2(&list));
}

fn largest<T: PartialOrd + Copy>(list: &[T]) -> T {
    let mut max_item = list[0];
    for &item in list.iter() {
        if item > max_item {
            max_item = item;
        }
    }
    max_item
}

fn largest2<T>(list: &[T]) -> T where
    T: PartialOrd + Copy {
    let mut max_item = list[0];
    for &item in list.iter() {
        if item > max_item {
            max_item = item;
        }
    }
    max_item
}