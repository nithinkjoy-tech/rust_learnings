fn main() {
    println!("Hello, world!{}",is_even(-21));
}

fn is_even(num : i32) -> bool {
    if num%2==0 {
        return true;
    }
    false
}
