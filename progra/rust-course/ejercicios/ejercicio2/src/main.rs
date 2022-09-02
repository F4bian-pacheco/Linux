
use std::io::{self, Write};

fn input(texto: &str) -> String{
    let mut entrada = String::new();
    print!("{}", texto);
    
    let _ = io::stdout().flush();
    io::stdin().read_line(&mut entrada).expect("No se puede leer");
    return entrada.trim().to_string();
}

fn to_int(texto: &str) -> i32 {
    return texto.parse::<i32>().unwrap();
}


fn main() {
    let numero: i32 = to_int(&input("Ingrese un numero: "));
    let cuadrado: i32 = numero * numero;
    let cubo: i32 = cuadrado * numero;

    println!("El cuadrado del numero {} es {}", numero, cuadrado);
    println!("El cubo del numero {} es {}", numero, cubo);
}
