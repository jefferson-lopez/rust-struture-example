#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        return self.width * self.height;
    }
    fn is_width(&self) -> bool {
        return self.width > 0;
    }
    fn is_height(&self) -> bool {
        return self.height > 0;
    }
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

fn main() {
    let scale = 2;
    let rect1 = Rectangle {
        width: dbg!(30 * scale),
        height: 50,
    };
    let rect2 = Rectangle {
        width: 10,
        height: 40,
    };

    dbg!(&rect1);

    println!(
        "The area of the rectangle is {} square pixels.",
        rect1.area()
    );
    println!("is Heigth {}", rect1.is_height());
    println!("is Width {}", rect1.is_width());
    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
}
