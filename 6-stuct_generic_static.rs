#[derive(Debug)]

struct Rect {
    height:f32,
    width:f32
}

impl Rect {
    fn area(&self) -> f32 {
        self.width*self.height
    }

    fn echo<T>(value: T) -> T {
        value
    }
}
fn main(){
    let rect= Rect {
        width:100.00,
        height:50.00,
    };

    println!("Area of rectangle is {}",rect.area());
    println!("Echoing {}",Rect::echo("Helloo"));
}
