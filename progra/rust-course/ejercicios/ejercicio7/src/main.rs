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

#[allow(dead_code)]
fn to_float(texto: &str) -> f32 {
    return texto.parse::<f32>().unwrap();
}


fn main() {
    const SECONDS_BY_DAY: i32 = 86400;
    let dias: i32 = to_int(&input("Ingrese el numero de dias: "));
    let segundos: i32 = dias * SECONDS_BY_DAY;

    println!("En {} dias hay {} segundos", dias, segundos);
}
