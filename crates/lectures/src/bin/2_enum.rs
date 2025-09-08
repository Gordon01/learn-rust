#[derive(Debug)] // Нужно чтобы напечатать значения
enum Shape {
    Circle(u16),
    Rectanlgle(u16, u16),
}

impl Shape {
    fn area(&self) -> u16 {
        match self {
            Shape::Circle(r) => 3 * r * r,
            Shape::Rectanlgle(a, b) => a * b,
        }
    }
}

fn main() {
    let circle = Shape::Circle(2);
    let rect = Shape::Rectanlgle(1, 2);

    println!("{circle:?} area = {}", circle.area());
    println!("{rect:?} area = {}", rect.area());
}
