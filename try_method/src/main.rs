struct Rectangle {
    width: u32,
    height: u32,
}
impl Rectangle {
    fn square(size: u32) -> Self {
        Self {
            width: size,
            height: size,
        }
    }

    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn width(&self) -> u32 {
        self.width
    }

    fn height(&self) -> u32 {
        self.height
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width >= other.width && self.height >= other.height
    }
}

fn main() {
    let rect1 = Rectangle {
        width: 50,
        height: 30,
    };
    let rect2 = Rectangle {
        width: 100,
        height: 200,
    };

    println!("The area of rect1 is {} square pixels.", rect1.area());
    println!("The area of rect2 is {} square pixels.", rect2.area());

    println!(
        "The width of rect1 is {}, the height of it is {}",
        rect1.width(),
        rect1.height()
    );

    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("Can rect2 hold rect1? {}", rect2.can_hold(&rect1));

    println!(
        "Area of square (11, 11) is {}",
        Rectangle::square(11).area()
    );
}
