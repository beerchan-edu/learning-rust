// Tuple Struct Exercise: Coordinate System

// You're building a 2D game and need to store positions and colors for objects.
// Create two tuple structs:

//     Position – stores the (x, y) coordinates of an object.
//     Color – stores an RGB color (red, green, blue).

// Requirements:

//     Define the two tuple structs.
//     Create two instances:
//         player_position at (10, 20).
//         player_color as (255, 0, 0) (Red).
//     Print each value by accessing the struct fields using dot notation.
//     Use destructuring to extract the values.
// Bonus Challenge 🌟

//    Write a function that takes a Position and moves it by a given (dx, dy), returning a new Position.

struct Position (i32, i32);

impl Position {
    fn move_by (&self, dx: i32, dy: i32) -> Self {
        Self(self.0 + dx, self.1 + dy)
    }
    
}
struct Color (i32, i32, i32);

pub fn coordinate_system() {
    let player_position = Position(10, 20);
    let player_color = Color(255, 0, 0);

    println!("Player is at position ({}, {})", 
            player_position.0, player_position.1
    );
    println!("Player color is ({}, {}, {})", 
            player_color.0, player_color.1, player_color.2
    );

    println!("Destructured Position: x = {}, y = {}", 
            player_position.0, player_position.1
    );
    println!("Destructured Color: R = {}, G = {}, B = {}", 
            player_color.0, player_color.1, player_color.2
    );

    let new_position = player_position.move_by(5, -7);

    println!("Position of the player moved to ({}, {})", new_position.0, new_position.1 )

}