#[derive(Debug)]

struct User {
    name:String,
    age: u8,
    email:String,
    is_active:bool,
}

fn main() {
    let user= User {
        name:String::from("Nithin K Joy"),
        age:25,
        email:String::from("nithinjoy@gmail.com"),
        is_active:true
    };

    print!("User data is {:?}",user);
}
