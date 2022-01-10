#[derive(Debug)]
struct Rect {
    width: u32,
    height: u32
}

impl Rect {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, other:&Rect)-> bool {
        self.width > other.width && self.height > other.height
    }

    fn squre(width: u32)-> Rect {
        Rect {
            width:width,
            height:width
        }
    }
}


fn main() {
    let rect1 = Rect {
        width: 32,
        height: 40
    };

    let rect2 = Rect {
        width: 33,
        height: 40
    };

    let rect3 = Rect {
        width: 12,
        height: 30
    };

    let rect4 = Rect::squre(40);
    println!("this rect area = {}",rect1.area());


    println!("this rect1 can hold rect2 ? = {}",rect1.can_hold(&rect2));
    println!("this rect2 can hold rect3 ? = {}",rect2.can_hold(&rect3));


    println!("this rect is squre {:?}",rect4)

}
