fn main(){
    let index=find_first_a(String::from("Nithin"));

    match index {
        Some(value)=>println!("The index is {}",value),
        None=>println!("Value not found")
    }
}

fn find_first_a(string:String) -> Option<i32> {
    for (index,char) in string.chars().enumerate() {
        if char =='a' {
            return Some(index as i32);
        }
    }
    return None;
}
