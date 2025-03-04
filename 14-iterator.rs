fn main(){
    let mut vec= vec![1,2,3,4,5];
    vec.iter_mut().for_each(|e| *e=*e*2);
    println!("{:?}",vec);
    
    let vec1= vec![1,2,3,4,5];
    for ele in vec1.iter().map(|e|e*5) {
        print!("{} ",ele);
    }
}
