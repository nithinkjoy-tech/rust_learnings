fn main() {
    fibonacci(10);
}

fn fibonacci(range: u32) {
    let mut first:i32=0;
    let mut second:i32=1;
    
    println!("{} \n{}",first,second);

    for _ in 0..range-2 {
        let temp=second;
        second=second+first;
        first=temp;
        println!("{}",second);
    }
}
