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
    
    const LIBRA_TO_KG: f32 = 0.45;
    const PIES_TO_MT: f32 = 0.3048;

    let nombre_dinosaurio: String = input("Ingrese el nombre del dinosaurio: ");
    let peso_dinosaurio: f32 = to_float(&input("Ingrese el peso del dinosaurio(en libras): "));
    let longitud_dinosaurio: f32 = to_float(&input("Ingrese la longitud del dinosaurio(en pies): "));
    
    let peso_dino_kg = peso_dinosaurio * LIBRA_TO_KG;
    let longitud_dino_mts = longitud_dinosaurio * PIES_TO_MT;

    print!("\nEl dinosaurio {} pesa {} kilos y mide {} metros", nombre_dinosaurio, peso_dino_kg, longitud_dino_mts);


}
