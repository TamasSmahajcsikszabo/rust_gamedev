fn main() {
    let rect = Rectangle { width:12, height: 12 };
    let rect2 = Rectangle { width:10, height: 6 };
    let rect3 = Rectangle { width:16, height: 5 };

    // println!("The rectangle area is {}", area(&rect));
    println!("The rectangle area is {}", rect.area());
    println!("The rectangle contains rectangle2 {}", rect.can_hold(&rect2));
    println!("The rectangle contains rectangle3 {}", rect.can_hold(&rect3));

    let sq=Rectangle::square(11);
    println!("Square with area {}", sq.area());
}
#[derive(Debug)]
struct Rectangle {
    width: u32, 
    height: u32
}
impl Rectangle {
    fn area(&self) -> u32 {
        // println!("Got input {:?}", rectangle);
        self.width * self.height
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }


    // associated function
    fn square(size: u32)->Rectangle {
        Rectangle{height: size, width: size}
    }
}


