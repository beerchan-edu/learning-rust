#[derive(Debug)]  // Adding the attribute to derive the Debug trait
struct Rectangle {
    width: u32,
    heihght: u32,
}

pub fn rectangles() {
    let scale = 2;
    let rect1 = Rectangle {
        width: dbg!(30 * scale),  // dbg! takes ownership of an expression and returns ownership of the value
        heihght: 50,
    };

//    println!("rect1 is {rect1:?}");  // :? printing the Rectangle instance using debug formatting
//    println!("rect1 is {rect1:#?}");  // :#? beautify printing using debug information

    dbg!(&rect1); // We don’t want dbg! to take ownership of rect1, so we use a reference here

    println!(  // prntln! only takes a reference of an expression
        "The area of the rectangle is {} square pixels.",
        area(&rect1)
    );
}

fn area(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.heihght
}