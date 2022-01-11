


fn main() {
    let mut v = vec![1, 2, 3, 4, 5];

    let v1 = &mut v[0];
    *v1 = 123;
    println!("{:?}",v);

    let third: &i32 = &v[2];
    println!("The third element is {}", third);

    match v.get(2) {
        Some(third) => println!("The third element is {}", third),
        None => println!("There is no third element."),
    }

    match v.get(5) {
        Some(third) => println!("The third element is {}", third),
        None => println!("There is no third element."),
    }
    let mut v = vec![100, 32, 57];
    for i in &mut v {
        *i += 50;
    }
    println!("{:?}", &v);


}
