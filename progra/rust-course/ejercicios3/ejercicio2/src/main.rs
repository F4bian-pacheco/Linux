
use std::io::{self, Write};

fn clear(){
    std::process::Command::new("clear").status().unwrap();
}
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

fn pares_impares(){
    clear();
    let mut numeros: Vec<i32> = Vec::new();
    for i in 0..10 {
        let entrada = to_int(&input(format!("ingrese el numero {}: ",i+1).as_str()));
        numeros.push(entrada);
    }
    
    let pares = numeros.clone().into_iter().filter(|n| n % 2 == 0).collect::<Vec<_>>();
    let impares = numeros.clone().into_iter().filter(|n| n % 2 != 0).collect::<Vec<_>>();

    println!("pares : {:?}", pares);
    println!("impares: {:?}", impares);

}

fn is_perfect(num: i32) -> bool {
    let mut suma: i32 = 0;

    for i in 1..num {
        if num % i == 0 {
            suma += i;
        }
    }
    return suma == num;
}


fn numero_perfecto(){
    clear();
    println!("100 numero perfectos: ");
    let mut i: i32 = 1;
    while i < 100{
        if is_perfect(i){
            print!("{} ",i);
        }
        i += 1;
    }
    println!("");
}

fn reverse_number(){
    clear();
    let mut entrada = to_int(&input("ingrese un numero de 4 digitos: "));
    let mut rev = 0;

    while entrada > 0 {
        let resto = entrada % 10;
        rev = rev*10 + resto;
        entrada = entrada/10;
    }

    println!("invertido: {}",rev);
}

fn main() {
    
    loop {
        println!("========Menu========");
        println!("1) pares e impares");
        println!("2) numeros perfectos");
        println!("3) numero invertido");
        println!("q) salir");
        let opcion = input("ingrese su opcion: ");
        if opcion == "q" {
            break;
        }else if opcion == "1" {
            pares_impares();
        }else if opcion == "2" {
            numero_perfecto();
        }else if opcion == "3" {
            reverse_number()
        }
    }
}
