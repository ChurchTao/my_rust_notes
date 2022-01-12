use std::collections::HashMap;

fn main() {
    map_create();

    array2map();
    
    read_map();

    entry_test();

}

///使用 entry 方法只在键没有对应一个值时插入
fn entry_test() {
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);

    scores.entry(String::from("Yellow")).or_insert(50);
    scores.entry(String::from("Blue")).or_insert(50);

    println!("{:?}", scores);
}

fn read_map() {
    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    let team_name = String::from("Blue");
    let score = scores.get(&team_name).unwrap();
    println!("{}",score);
}

fn array2map() {
    let vector1 = vec![1,2,3];
    let vector2 = vec![String::from("helloworld"),String::from("value2"),String::from("value3")];

    let map:HashMap<_,_> = vector2.iter().zip(vector1.iter()).collect();
    println!("output map2 {:?}",map);

}

fn map_create(){
    let mut map1 = HashMap::new();
    map1.insert("hello","world");
    map1.insert("key2","value2");
    map1.insert("key3","value3");

    println!("output map1 {:?}",map1);
}
