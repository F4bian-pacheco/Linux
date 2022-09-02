
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

fn contiene(texto: &str, palabra: &str) -> bool {

    for i in texto.split_whitespace() {
        if i == palabra {
            return true;
        }
    }
    return false;
}

fn contiene_2(texto: &str, palabra: &str) -> bool {
    for i in texto.split_whitespace() {
        if String::from(palabra).to_lowercase() == String::from(i).to_lowercase(){
            return true;
        }
        
    }
    return false;
}


fn main() {
    let texto = input("ingrese el texto: ");
    let palabra = input("ingrese la palabra: ");
    
    if texto.contains(&palabra){
        println!("palabra encontrada");
    }else{
        println!("palabra no encontrada")
    }
    println!("{}",contiene(&texto,&palabra));
    println!("{}",contiene_2(&texto,&palabra));

}
