struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

// can have multiple impl blocks for a struct
impl Rectangle {
    fn square(size: u32) -> Self {
        Self { width: size, height: size }
    }
}

fn main() {
    let mut user1 = User {
        email: String::from("john.doe@example.com"),
        username: String::from("john_doe"),
        sign_in_count: 1,
        active: true,
    };

    let name = user1.username;

    user1.username = String::from("john_doe_2");


    let user2 = build_user(
        String::from("jane.doe@example.com"),
        String::from("jane_doe"),
    );

    let user3 = User {
        email: String::from("john.doe@example.com"),
        username: String::from("john_doe"),
        ..user1 // spread the remaining fields from user1
    };

    // tuple struct
    struct Color(i32, i32, i32);
    struct Point(i32, i32, i32);

    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);

    
    let rect = Rectangle { width: 30, height: 50 };
    println!("rect is {rect:?}");

    let area = rect.area();
    println!("area is {area}");

    let rect2 = Rectangle { width: 20, height: 40 };
    println!("rect2 is {rect2:?}");

    let can_hold = rect.can_hold(&rect2);
    println!("rect can hold rect2: {can_hold}");

    let square = Rectangle::square(20);
    println!("square is {square:?}");
}

// shorthand syntax for creating a new struct
fn build_user(email: String, username: String) -> User {
    User {
        email,
        username,
        sign_in_count: 0,
        active: true,
    }
}

fn area(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}
