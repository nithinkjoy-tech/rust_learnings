fn main(){
    let mut vec= Vec::new();

    vec.push(1);
    vec.push(2);
    vec.push(3);
    vec.push(4);
    vec.push(5);
    vec.push(6);

    // let new_vec=return_even(vec);
    let new_vec=return_even(&vec);
    println!("{:?}",new_vec);
    println!("{:?}",vec);
}

// fn return_even(vec: Vec<u32>) -> Vec<u32>{
//     return vec.into_iter().filter(|e|e%2==0).collect();
// }

fn return_even(vec: &Vec<u32>) -> Vec<u32>{
    return vec.iter().filter(|&e|e%2==0).cloned().collect();
}
