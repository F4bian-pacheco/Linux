

fn f(x: i32) -> i32 {
    let y: i32 = x.pow(3) + 2*x.pow(2) + 3*x + 2;
    y
}



fn main() {

    let res = f(3);
    println!("x^3 + 2*x^2 + 3*x +5 = {}",res)
}
