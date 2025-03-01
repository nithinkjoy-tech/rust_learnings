fn main() {
    let string = String::from("Hello World!");
    let string_length:usize= get_string_length(string);
    print!("String length is {}",string_length);
}

fn get_string_length(string: String) -> usize {
    string.chars().count()
}
