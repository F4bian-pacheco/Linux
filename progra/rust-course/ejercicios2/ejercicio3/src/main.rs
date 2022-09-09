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

fn max(vector: &Vec<i64>){
    let mut maxi : i64 = 0;
    for i in vector {
        if i > &maxi {
            maxi = *i;
        }
    }
    println!("mayor {}", maxi);
}

fn min(vector: &Vec<i64>){
    let mut min : i64 = 9999999999;
    for i in vector {
        if i < &min{
            min = *i
        }
    }
    println!("menor {}", min);
}

fn main() {
    let mut vector: Vec<i64> = Vec::new();

    loop {
        let entrada = to_int(&input("ingrese un numero: "));
        if entrada == 0 {
            break;
        }
        vector.push(entrada);
        println!("{:?}",vector);
        max(&vector);
        min(&vector);
    }
}
