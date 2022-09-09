
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
    let mut ulam_number: i32 = to_int(&input("ingrese un numero: "));
    
    while ulam_number != 1 {
        if ulam_number % 2 == 0 {
            ulam_number = ulam_number / 2;
        }else{
            ulam_number = (ulam_number * 3) + 1;
        }
        println!("{}",ulam_number);
    }
}
