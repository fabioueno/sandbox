fn main() {
    let scale = 2;

    let r1 = Rectangle {
        width: dbg!(30 * scale),
        height: 50,
    };

    println!("The area of the rectangle is {} square pixels.", r1.area());

    dbg!(&r1);

    let r2 = Rectangle { width: 30, height: 50 };
    let r3 = Rectangle { width: 10, height: 40 };
    let r4 = Rectangle { width: 60, height: 45 };

    println!("Can r2 hold r3? {}", r2.can_hold(&r3));
    println!("Can r3 hold r4? {}", r3.can_hold(&r4));

    let s1 = Rectangle::square(3);
    dbg!(&s1);
}

#[derive(Debug)]
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
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, other: &Self) -> bool {
        self.width > other.width && self.height > other.height
    }
}