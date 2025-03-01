use std::fs;

fn main(){
    let result=fs::read_to_string("./src/nithin.txt");

    match result {
        Ok(data)=>println!("{}",data),
        Err(err)=>println!("{}",err)
    }
}
