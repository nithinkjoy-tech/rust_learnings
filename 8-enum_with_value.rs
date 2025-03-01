#[derive(Debug)]

enum Shape {
    Circle(f32),
    Square(f32),
    Rectangle(f32,f32)
}

fn main() {
    let circle=Shape::Circle(10.5);
    let square=Shape::Square(20.00);
    let rectangle=Shape::Rectangle(100.00,50.00);

    println!("{:?}",calculate_area(circle));
    println!("{:?}",calculate_area(square));
    println!("{:?}",calculate_area(rectangle));
}

fn calculate_area(shape: Shape) -> f32{
    match shape {
        Shape::Circle(r)=>3.14*r*r,
        Shape::Square(a)=>a*a,
        Shape::Rectangle(a,b)=>a*b
    }
}
