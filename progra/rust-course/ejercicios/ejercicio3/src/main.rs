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
    let lado_a: f32 = to_float(&input("Ingrese el valor del lado A: "));
    let lado_b: f32 = to_float(&input("Ingrese el valor del lado B: "));

    let pow_a = lado_a.powf(2.0);
    let pow_b = lado_b.powf(2.0);

    let hipotenusa = (pow_a + pow_b).sqrt();

    println!("La hipotenusa vale: {}", hipotenusa);
}
