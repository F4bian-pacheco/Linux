use std::fs::File;
use std::path::Path;
use std::fs::OpenOptions;
use std::io::Write;
use std :: io ::{ self , BufRead };
use std::io::stdin;

fn input() -> String {
    let mut nuevo: String = String::new();
    stdin().read_line(&mut nuevo).unwrap();
    let nuevo2: String = nuevo.trim().to_owned();
    return nuevo2
}

#[warn(dead_code)]
struct Producto {
    nombre:String,
    precio:String,
}

struct Reserva {
    nombre_producto:String,
    nombre_cliente:String,
    hora_retiro:String
}

fn leer<P>(nombre: P) -> io::Result<io::Lines<io::BufReader<File>>> where P: AsRef<Path>, {
    let file = File::open(nombre)?;
    Ok(io::BufReader::new(file).lines())
}

fn modificar_archivo(p: &Path, t: String) {
    let mut file = match File::create(p) {
        Err(why) => panic!("couldn't create {}",  why),
        Ok(file) => file,
    };
    file.write_all(t.as_bytes());
}



fn agregar_archivo(p: &Path) -> File{
    let mut binding = OpenOptions::new();
    let binding = binding.append(true);
    let file = match binding.open(p){
        Err(why) => panic!("No se puede abrir el archivo"),
        Ok(file) => file,
    };
    return file
}

fn reservar() {
    let path_reserva = Path::new("reservas.txt");
    println!("Ingrese nombre del producto:");
    let nombre_producto: String = input();
    
    println!("Ingrese su nombre :");
    let nombre_cliente: String = input();
    println!("Ingrese hora de retiro:");
    let hora_retiro: String = input();
    
    let m = nueva_reserva(nombre_producto, nombre_cliente, hora_retiro);
    let temp = format!("{},{},{}\n", m.nombre_cliente, m.nombre_producto, m.hora_retiro);
    let mut file = agregar_archivo(path_reserva);
    file.write_all(temp.as_bytes());
    println!("ha sido añadido con éxito.");
}

fn nueva_reserva(nom_p: String, nom_c:String, hora_retiro:String) -> Reserva{
    let m = Reserva{
        nombre_cliente:String::from(nom_c),
        nombre_producto:String::from(nom_p),
        hora_retiro:String::from(hora_retiro)
    };
    return m
}

fn nuevo_producto(nom: String, preci: String) -> Producto {
    let m = Producto {
        nombre:String::from(nom),
        precio:String::from(preci),
    };
    return m
}

fn consulta_precio(p: &Path) {
    println!("Ingrese el nombre del producto a consultar:");
    let dato: String = input();
    let mut comp: bool = false;
    if let Ok(lines) = leer(p) {
        for line in lines {
            let linea: String = line.unwrap();
            let mut item: [&str; 2] = [""; 2];
            for (i, elem) in linea.split(",").enumerate() {
                item[i] = elem;
            }
            if dato == item[0] || dato == item[1] {
                println!("El precio del item consultado({}) es de:\n{} pesos.", item[0], item[1]);
                comp = true;
                break
            } else {
                continue
            }
        }
        if comp == false {
            println!("Este item no se encuentra en la base de datos.");
        }
    }
}

fn listado_productos(p: &Path) {
    let mut temp: String = String::from("Los productos del kiosko caibi son:\n");
    if let Ok(lines) = leer(p) {
        for line in lines {
            let linea: String = line.unwrap();
            let mut item: [&str; 2] = [""; 2];
            for (i, elem) in linea.split(",").enumerate() {
                item[i] = elem;
            }
            println!("{}: ${}",item[0],item[1]);
        }
            
        if temp == "Los productos del kiosko caibi son:\n" {
            println!("Este producto no se encuentra en la base de datos.")
        } else {
            println!("{}", &temp);
        }
    }
}


fn identificar(dato: String) -> usize {
    let mut index: usize = 0;
    if dato == "1" {
        index = 0;
    } else if dato == "2" {
        index = 1;
    }
    return index
}

fn editar_archivo(p: &Path) {
    println!("esta decision es valido solo para el personal de negocion, por favor ingrese el codigo maestro ");
    let mut clave: String = input();
    while  clave == "icb"{
        println!("Ingrese el nombre del Producto:");
        let nombre: String = input();
        let mut cont: u8 = 0;
        let mut temp: String = String::new();
        if let Ok(lines) = leer(p) {
            for line in lines {
                let linea: String = line.unwrap();
                let mut item: [&str; 2] = [""; 2];
                for (i, elem) in linea.split(",").enumerate() {
                    item[i] = elem;
                }
                if nombre == item[0] {
                    let mut temp2: String = String::new();
                    let dato: String = dec2(); // 1 para nombre, 2 para precio
                    println!("Ingrese la modificación:");
                    let modi: String = input(); 
                    let index: usize = identificar(dato);
                    item[index] = &modi;
                    for i in 0..2 {
                        if i == 1 {
                            temp2 = format!("{}{}", temp2, item[i]);
                        } else {
                            temp2 = format!("{}{},", temp2, item[i]);
                        }
                    }
                    temp = format!("{}{}\n", temp, temp2);
                    cont += 1;
                } else {
                    temp = format!("{}{}\n", temp, linea);
                }
            }
            if cont == 0 {
                println!("Este Producto no se encuentra en la base de datos.");
            } else {
                modificar_archivo(p, temp);
                println!("La base de datos a sido modificada con éxito.");
                break
            }
        }else{
            println!("Ocurrio un error al ingresar al archivo");
        }
    }
    
}
//modificar este funcion(recordatorio)
fn dec2() -> String {
    println!("Ingrese el dato que desea modificar:\n1: Nombre del producto.\n2: Precio de venta.\n");
    let mut dato: String = input();
    while dato != "1" && dato != "2" {
        println!("Por favor ingrese un número entre 1 y 2");
        dato = input();
    }
    return dato
}

fn eliminar_producto(p: &Path) {
    println!("Ingrese un producto que desea eliminar:");
    let nombre: String = input();
    let mut cont: u8 = 0;
    let mut temp: String = String::new();

    if let Ok(lines) = leer(p) {
        for line in lines {
            let linea: String = line.unwrap();
            let mut item: [&str; 2] = [""; 2];
            for (i, elem) in linea.split(",").enumerate() {
                item[i] = elem;
            }
            if nombre == item[0] {
                cont += 1;
            } else {
                temp = format!("{}{}\n", temp, linea);
            }
        }
    }
    if cont == 0 {
        println!("Este producto no se encuentra en la base de datos.");
    } else {
        modificar_archivo(p, temp);
        println!("La base de datos a sido modificada con éxito.");
    }
}

fn menu() -> String {
    println!("Bienvenido al Kiosko Caibi, por favor seleccione una de las siguientes opciones:
\x1b[93m 1: Ingresar una reserva.\x1b[0m
\x1b[93m 2: Consultar el precio de un producto.\x1b[0m
\x1b[93m 3: Listar los productos.\x1b[0m
\x1b[32m 4: Modificar un elemento de un producto.\x1b[0m
\x1b[31m 5: Eliminar un producto de la base de datos.\x1b[0m");
    let mut decision: String = input();
    while decision != "1" && decision != "2" && decision != "3" 
    && decision != "4" && decision != "5" {
        println!("Por favor ingrese un número entre 1 y 5");
        decision = input();
    }
    return decision
}

fn camino(path: &Path, decision: &str) {
    if decision == "1"{
        reservar();
    } else if decision == "2" {
        consulta_precio(path);
    } else if decision == "3"{
        listado_productos(path);
    } else if decision == "4"{ 
        editar_archivo(path);
    } else if decision == "5" {
        eliminar_producto(path);
    }
}

fn main() {
    let path = Path::new("kiosko_caibi.txt");
    let decision: String = menu();
    camino(path, &decision);
}


