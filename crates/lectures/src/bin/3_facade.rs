mod geometry {
    use std::f32::consts::PI;

    pub struct Circle(pub f32);
    pub struct Rectangle(pub f32, pub f32);

    pub trait Area {
        fn area(&self) -> f32;
    }

    impl Area for Circle {
        fn area(&self) -> f32 {
            PI * self.0 * self.0
        }
    }

    impl Area for Rectangle {
        fn area(&self) -> f32 {
            self.0 * self.1
        }
    }
}

use geometry::Area;

fn main() {
    // use {Circle, Rectangle};

    let cirle = geometry::Circle(2.0);
    let rect = geometry::Rectangle(1.0, 2.0);

    print_area(cirle);
    print_area(rect);
}

fn print_area(shape: impl Area) {
    println!("Area: {:.2}", shape.area())
}
