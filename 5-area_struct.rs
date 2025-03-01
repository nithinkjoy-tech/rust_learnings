#[derive(Debug)]

struct Rect {
    height:f32,
    width:f32
}

impl Rect {
    fn area(&self) -> f32 {
        self.width*self.height
    }
}
fn main(){
    let rect= Rect {
        width:100.00,
        height:50.00,
    };

    print!("Area of rectangle is {}",rect.area());
}
