


fn leer_entrada() -> String{
    let mut line = String::new();
    let _b1 = std::io::stdin().read_line(&mut line).unwrap();
    
    return line[0..line.len() - 1].to_string();
}


fn main() {
    println!("ingrese algo: ");
    let entrada = leer_entrada();
    let num1 :i32 = 20;
    println!("ud ha ingresado {}", entrada);
    let entrada_int = entrada.parse::<i32>().unwrap();
    let res = entrada_int + num1;
    println!("La suma es: {}",res);
}
