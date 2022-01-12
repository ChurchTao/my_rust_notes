fn main() {
    println!("Hello, world!");
    let list = vec![10,21,7,-1,33,43];
    println!("Hello, world! max = {}",largest(&list));
    let list = vec!['c','a','b','1','l','z'];
    println!("Hello, world! max = {}",largest(&list));
}

fn largest<T: std::cmp::PartialOrd>(list: &[T]) -> &T {
    let mut max_item = &list[0];
    for item in list.iter() {
        if max_item < item {
            max_item = item;
        }
    }
    return max_item;
}