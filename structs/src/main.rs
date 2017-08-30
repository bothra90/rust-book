#[derive(Debug)]

// Rectangle is a struct
struct Rectangle {
    length: u32,
    width: u32,
}

impl Rectangle {
    // methods have a default 1st argument of '&self'
    // Methods can choose to take ownership of self, borrow self immutably as we’ve done here, or
    // borrow self mutably, just like any other parameter.
    fn area(&self) -> u32 {
        self.length * self.width
    }
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.length > other.length && self.width > other.width
    }
    fn take_own(self) {
        // Do nothing
    }
}

// Separate impl block
impl Rectangle {
    fn square(size: u32) -> Rectangle {
        Rectangle {
            length: size,
            width: size,
        }
    }
}


fn main() {
    let rect1 = Rectangle {
        length: 50,
        width: 30,
    };
    println!(
        "The area of the rectangle is {} square pixels.",
        rect1.area()
    );
    rect1.take_own();
    // This call is invalid
    // rect1.area();
    //
    // Calling a associative (static?) function.
    let sq = Rectangle::square(3);
    println!("Area of the square: {}", sq.area());
}
