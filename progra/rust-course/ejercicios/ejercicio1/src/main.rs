
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
    println!("-----Bienvenido Utalino-----");
    //print!("Ingrese el Rut del Estudiante: ");

    let rut = input("Ingrese el Rut del Estudiante: ");
    //let num1 = to_int(&input("Ingrese un numero: "));
    //let num2 = to_int(&input("Ingrese un numero: "));
    
    // println!("suma: {}", num1+num2);

    // println!("ingresaste {:?}", rut);
    
    const NUM_NOTAS: i16 = 3;
    let mut i: i16 = 0;
    let mut res: f32 = 0.0;
    loop {
        let aux: f32 = to_float(&input(format!("Ingrese nota {}: ", i+1).as_str()));
        res = res + aux;
        i = i + 1;
        if i == NUM_NOTAS {
            break;
        }
        
    }
    res = res / 3.0;
    println!("El promedio del Alumno {} es {:.2}",rut, res);
    
}
