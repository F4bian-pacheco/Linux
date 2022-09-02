use std::io::{self, Write};

fn input(texto: &str) -> String{
    let mut entrada = String::new();
    print!("{}", texto);
    
    let _ = io::stdout().flush();
    io::stdin().read_line(&mut entrada).expect("No se puede leer");
    return entrada.trim().to_string();
}

#[allow(dead_code)]
fn to_int(texto: &str) -> i32 {
    return texto.parse::<i32>().unwrap();
}

fn to_float(texto: &str) -> f32 {
    return texto.parse::<f32>().unwrap();
}



fn main() {
    let base: f32 = to_float(&input("ingrese el valor de la base: "));
    let altura: f32 = to_float(&input("ingrese el valor de la altura: "));

    let perimetro: f32 = 2.0 * (base + altura);
    let superficie: f32 = base * altura;

    println!("El perimetro del rectangulo es: {}",perimetro);
    println!("La superficie del rectangulo es: {}", superficie);
}
