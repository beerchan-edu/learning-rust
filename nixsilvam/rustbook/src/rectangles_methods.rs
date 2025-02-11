#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    // All functions defined within an impl block are called associated functions because they’re associated with the type named after the impl
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn width(&self) -> bool {
        self.width > 0
    }

    // Associated functions that aren’t methods are often used for constructors that will return a new instance of the struct
    fn square(size: u32) -> Self {
        Self {
            width: size,
            height: size
        }
    }
}

// Each struct is allowed to have multiple impl blocks
impl Rectangle {
    // Implementing the can_hold method on Rectangle that takes another Rectangle instance as a parameter
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
    
}

pub fn rectangles() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };
    let rect2 = Rectangle {
        width: 10,
        height: 40,
    };
    let rect3 = Rectangle {
        width: 60,
        height: 45,
    };

    // To call this associated function, we use the :: syntax with the struct name
    let sq = Rectangle::square(3);

    println!("The area of the rectangle is {} square pixels.",
            rect1.area()
    );

    if rect1.width() {
        println!("The rectangle has a nonzero width; it is {}", rect1.width);
    }

    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));

    println!("This rectangle is a real square with the area of {} pixels",
            sq.area()
    );
}