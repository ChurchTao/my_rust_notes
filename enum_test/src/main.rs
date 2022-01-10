
// 可以视为简单类
#[derive(Debug)]
enum IpAddr {
    V2(u8,u8,u8,u8),
    V4(String),
    V6(String)
}

fn main() {

    let home_v2 = IpAddr::V2(127,0,0,1);
    let home_v4 = IpAddr::V4(String::from("127.0.0.1"));
    let home_v6 = IpAddr::V6(String::from("::1"));

    // let a = std::net::IpAddr::V4(std::net::Ipv4Addr::new(127, 0, 0, 1));
    println!("ipv2 = {:?} ,ipv4 = {:?} , ipv6 = {:?}",home_v2,home_v4,home_v6);

    let obj = Some(1);
    let obj2 = Some("String of str");
    let absent_number: Option<i32> = None;

}


enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}