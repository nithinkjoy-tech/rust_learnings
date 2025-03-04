// This is same like abstraction in java

trait Summarise {
    fn summarise(&self) -> String;
}

struct User {
    name:String,
    age:u32
}

impl Summarise for User {
    fn summarise(&self) -> String {
        return format!("Name is {} and age is {}",self.name,self.age);
    }
}


fn main(){
    let user=User {
        name:String::from("Nithin"),
        age:25
    };

    let summary=user.summarise();
    println!("{}",summary);
}

