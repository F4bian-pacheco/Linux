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
    const GALON_TO_LITRO: f32 = 3.785;
    const PRECIO_LITRO: f32 = 820.0;

    let galones: f32 = to_float(&input("Cuantos galones desea llevar?: "));

    let precio_total = (galones * GALON_TO_LITRO) * PRECIO_LITRO;

    println!("El precio de los galones es: {}", precio_total);
    
}
