fn main() {
    let mut vec = Vec::new();
    vec.push(1);
    vec.push(2);
    println!("{:?}", vec);
    let x = &vec[0];
    vec.push(3); // this will cause a bug
    println!("x = {}", x);
}