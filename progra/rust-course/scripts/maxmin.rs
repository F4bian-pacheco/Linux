use std::{io::{self, Write}};


fn input(texto: &str) -> String{
    let mut entrada = String::new();
    print!("{}", texto);
    
    let _ = io::stdout().flush();
    io::stdin().read_line(&mut entrada).expect("No se puede leer");
    return entrada.trim().to_string();
}

fn to_int(texto: &str) -> i64 {
    return texto.parse::<i64>().unwrap();
}


fn main(){
    let mut numeros = String::new();

    loop {
       let entrada = input("ingrese un numero: ");
       if (entrada) == "0"{
           break;
       }
       numeros += entrada.as_str();
    }
    println!("{}",numeros);
}
