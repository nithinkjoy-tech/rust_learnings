use std::collections::HashMap;

fn main(){
    let mut map:HashMap<String,i32>=HashMap::new();
    map.insert(String::from("id"), 20);
    map.insert(String::from("age"), 25);

    let value=map.get("age");
    match value {
        Some(val)=>println!("Age is {}",val),
        None=>println!("The given key does not exist")
    }
}
