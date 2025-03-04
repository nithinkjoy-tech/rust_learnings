fn main(){
    let mut vec= vec![1,2,3,4,5];

    for item in vec.iter_mut() {
        *item=*item*2;
    }

    println!("{:?}",vec);
}
