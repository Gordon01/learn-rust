struct Circle(u16);
struct Rectangle(u16, u16);

trait Area {
    fn area(&self) -> u16;
}

impl Area for Circle {
    fn area(&self) -> u16 {
        3 * self.0 * self.0
    }
}

impl Area for Rectangle {
    fn area(&self) -> u16 {
        self.0 * self.1
    }
}

fn main() {
    let cirle = Circle(2);
    let rect = Rectangle(1, 2);

    print_area(cirle);
    print_area(rect);
}

fn print_area(shape: impl Area) {
    println!("Area: {}", shape.area())
}
