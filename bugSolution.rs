fn main() {
    let mut vec = Vec::new();
    vec.push(1);
    vec.push(2);
    println!("{:?}", vec);
    
    // Create a copy instead of a reference
    let x = vec[0];
    vec.push(3);
    println!("x = {}", x);
    
    //Alternatively, use iterators or other methods that do not invalidate references
    // ...
} 