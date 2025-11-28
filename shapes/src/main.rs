fn main() {
    println!("This is a simple program to play with enums with associated data");

    let circle = Shape::Circle(10.0);
    let rectangle = Shape::Rectangle(10.0, 20.0);
    let square = Shape::Square(10.0);
    let triangle = Shape::Triangle(3.0, 4.0, 5.0);

    println!("Area of circle: {}", circle.calculate_area());
    println!("Area of rectangle: {}", rectangle.calculate_area());
    println!("Area of square: {}", square.calculate_area());
    println!("Area of triangle: {}", triangle.calculate_area());
}

enum Shape {
    Circle(f64),
    Rectangle(f64, f64),
    Square(f64),
    Triangle(f64, f64, f64),
}

impl Shape {
    fn calculate_area(&self) -> f64 {
        match self {
            Shape::Circle(radius) => radius * radius * std::f64::consts::PI,
            Shape::Rectangle(width, height) => width * height,
            Shape::Square(side) => side * side,
            Shape::Triangle(a, b, c) => {
                let s = (a + b + c) / 2.0;
                (s * (s - a) * (s - b) * (s - c)).sqrt()
            }
        }
    }
}