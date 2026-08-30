use std::ops::Add;

#[derive(Debug, Copy, Clone, PartialEq)]
struct Point<T> {
    x: T,
    y: T,
}

impl<T> Point<T> {
    fn new(x: T, y: T) -> Self {
        Self { x, y }
    }
}

impl<T: Add<Output = T>> Add for Point<T> {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

fn main() {
    let point = Point::new(1, 2);
    println!("Point integer: ({}, {})", point.x, point.y);

    let point_f = Point::new(1.0, 2.0);
    println!("Point1 float: ({}, {})", point_f.x, point_f.y);

    let point_f_2 = Point::new(3.0, 4.0);
    println!("Point2 float: ({}, {})", point_f_2.x, point_f_2.y);

    // Nyt tämä +merkki toimii loistavasti!
    let point_f_sum = point_f + point_f_2;
    println!("Point sum float: ({}, {})", point_f_sum.x, point_f_sum.y);
}