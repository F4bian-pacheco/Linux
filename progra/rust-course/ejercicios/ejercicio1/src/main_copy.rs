

#[allow(dead_code)]
fn leer_entrada() -> String{
    let mut line = String::new();
    let _b1 = std::io::stdin().read_line(&mut line).unwrap();
    
    return line[0..line.len() - 1].to_string();
}


fn main() {
    println!("ingresa algo: ");
    let entrada = leer_entrada();
    println!("ingresaste {:?}", entrada);
}
