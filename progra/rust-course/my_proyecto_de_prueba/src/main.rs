fn main() {
    println!("Hello, world!");
    
    let v = vec![100,20,5];

    for i in &v {
        println!("{}",i);
    }
}
