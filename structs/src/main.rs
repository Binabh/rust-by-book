#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}
impl Rectangle{
    fn area(&self)->u32{
        self.width * self.height
    }
    fn can_hold(&self, other: &Rectangle)->bool{
        self.width > other.width && self.height > other.height
    }
    fn square(size: u32)-> Self{
        Self {
            width: size,
            height: size,
        }
    }
    fn is_square(&self)-> bool {
        self.height == self.width
    }
}
fn main() {
    let rect1 = Rectangle{
        width: 30,
        height: 50,
    };
    let rect2 = Rectangle{
        width: 10,
        height: 10,
    };
    let sq1 = Rectangle::square(50);
    println!("rect1 is {rect1:?}");
    println!("rect2 is {rect2:?}");
    println!("can rect2 hold rect1? {}", rect2.can_hold(&rect1));
    println!("area of rect1 is {}", rect1.area());
    println!("is sq1 square? {}", sq1.is_square());
}
