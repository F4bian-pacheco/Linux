
use std::io::{self, Write};

fn input(texto: &str) -> String{
    let mut entrada = String::new();
    print!("{}", texto);
    
    let _ = io::stdout().flush();
    io::stdin().read_line(&mut entrada).expect("No se puede leer");
    return entrada.trim().to_string();
}

fn most_large(palabra1: &str, palabra2:&str){
    if palabra1.len() > palabra2.len(){
        let len_diff: usize = palabra1.len() - palabra2.len();
        println!("{} es mas grande que {} por {} letras",palabra1,palabra2,len_diff);
    }else{
        let len_diff: usize = palabra2.len() - palabra1.len();
        println!("{} es mas grande que {} por {} letras",palabra2, palabra1, len_diff);
    }

}


fn main() {
    let palabra1 = input("ingrese la primera palabra: ");
    let palabra2 = input("ingrese la segunda palabra: ");

    most_large(&palabra1,&palabra2);

}
