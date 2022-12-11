use std::fs::File;
use std::fs::OpenOptions;
use std::io::stdin;
use std::io::Write;
use std::io::{self, BufRead};
use std::path::Path;
use std::process;

fn input() -> String {
    let mut nuevo: String = String::new();
    stdin().read_line(&mut nuevo).unwrap();
    let nuevo2: String = nuevo.trim().to_owned();
    return nuevo2;
}

#[warn(dead_code)]
struct Producto {
    nombre: String,
    precio: String,
    stock: String, 
}

struct Reserva {
    nombre_producto: String,
    cantidad_producto: String,
    nombre_cliente: String,
    hora_retiro: String,
    dia_retiro: String,
}

fn leer<P>(nombre: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(nombre)?;
    Ok(io::BufReader::new(file).lines())
}

fn modificar_archivo(p: &Path, t: String) {
    let mut file = match File::create(p) {
        Err(why) => panic!("couldn't create {}", why),
        Ok(file) => file,
    };
    file.write_all(t.as_bytes());
}

fn agregar_archivo(p: &Path) -> File {
    let mut binding = OpenOptions::new();
    let binding = binding.append(true);
    let file = match binding.open(p) {
        Err(why) => panic!("No se puede abrir el archivo"),
        Ok(file) => file,
    };
    return file;
}

fn verificacion_horario(dia: String, hora: String )-> bool {
    let path_horario = Path::new("horario.txt");
    let mut correcto = false;
    if let Ok(lines) = leer(path_horario) {
        for line in lines {
            let linea: String = line.unwrap();
            let info_horario = linea.split(",").collect::<Vec<&str>>();
            //println!("{:?}, {}", info_horario, info_horario[0]);
            if dia.to_lowercase() == info_horario[0].to_lowercase() {
                println!("{}", info_horario[0]);
                let inicio = info_horario[1].split(":").collect::<Vec<&str>>();
                let termino = info_horario[2].split(":").collect::<Vec<&str>>();
                let hora_inicio = inicio[0].to_string().parse::<i32>().unwrap();
                let hora_termino = termino[0].to_string().parse::<i32>().unwrap();
                let hora_comparar = hora.split(":").collect::<Vec<&str>>()[0];
                let hora_comparar = hora_comparar.to_string().parse::<i32>().unwrap();
                if hora_comparar >= hora_inicio && hora_comparar < hora_termino {
                    correcto = true;
                }
            }
        }
    }
    return correcto;
    
}

fn reservar() {
    let path_reserva = Path::new("reservas.txt");
    let path_productos = Path::new("kiosko_caibi.txt");
    let mut contador = 0;
    //println!("Ingrese nombre del producto:");
    let mut nombre_producto: String = String::from(" ");
    //nombre_producto = nombre_producto.to_lowercase();
    while contador == 0 {
        println!("Ingrese nombre del producto:");
        nombre_producto = input();
        nombre_producto = nombre_producto.to_lowercase();
        if let Ok(lines) = leer(path_productos) {
            let lineas: Vec<_> = lines.collect::<Result<_,_>>().unwrap(); 
            println!("{:?}", lineas);
            for producto in lineas {
                if producto.to_lowercase().contains(&nombre_producto) {
                    contador+= 1;
                    
                }
                
            }
            if contador == 0 {
                println!("");
                println!("El producto ingresado no existe en el kiosco caibi");
                println!("");
                
                
            }
    
            
        }
    }  

    println!("Ingrese su nombre:");
    let mut nombre_cliente: String = input();
    nombre_cliente = nombre_cliente.to_lowercase();
    println!("Ingrese hora de retiro (formato 00:00):");
    let hora_retiro: String = input();
    println!("Ingrese dia de retiro:");
    let mut dia_retiro: String = input();
    dia_retiro = dia_retiro.to_lowercase();
    
    while !verificacion_horario(dia_retiro.clone(),hora_retiro.clone()) {
        println!("hora o dia son invalidos, intente nuevamente");
        println!("Ingrese hora de retiro (formato 00:00):");
        let hora_retiro: String = input();
        println!("Ingrese dia de retiro:");
        let mut dia_retiro: String = input();
        dia_retiro = dia_retiro.to_lowercase();
    }

    println!("Ingrese la cantidad:");
    let cantidad_producto: String = input();

    let m = nueva_reserva(nombre_producto, nombre_cliente, hora_retiro, dia_retiro, cantidad_producto);
    let temp = format!(
        "{},{},{},{},{}\n",
        m.nombre_cliente, m.nombre_producto, m.hora_retiro, m.dia_retiro, m.cantidad_producto
    );
    let mut file = agregar_archivo(path_reserva);
    file.write_all(temp.as_bytes());
    println!("ha sido añadido con éxito.");
}

fn nueva_reserva(nom_p: String, nom_c: String, hora_retiro: String, dia_retiro: String, cantidad_producto: String) -> Reserva {
    let m = Reserva {
        nombre_cliente: String::from(nom_c),
        nombre_producto: String::from(nom_p),
        hora_retiro: String::from(hora_retiro),
        dia_retiro: String::from(dia_retiro),
        cantidad_producto: String::from(cantidad_producto),
    };
    return m;
}

fn nuevo_producto(nom: String, preci: String, stock: String) -> Producto {
    let m = Producto {
        nombre: String::from(nom),
        precio: String::from(preci),
        stock: String::from(stock), 
    };
    return m;
}

fn consulta_precio(p: &Path) {
    println!("Ingrese el nombre del producto a consultar:");
    let mut dato: String = input();
    dato = dato.to_lowercase();
    let mut comp: bool = false;
    if let Ok(lines) = leer(p) {
        for line in lines {
            let linea: String = line.unwrap();
            let mut item: [&str; 3] = [""; 3];
            for (i, elem) in linea.split(",").enumerate() {
                item[i] = elem;
            }
            if dato == item[0] || dato == item[1] {
                println!("");
                println!(
                    "El precio del producto consultado({}) es de:\n{} pesos.",
                    item[0], item[1]
                );
                println!("");
                comp = true;
                break;
            } else {
                continue;
            }
        }
        if comp == false {
            println!("");
            println!("Este producto no se encuentra en la base de datos.");
            println!("");
            consulta_precio(p);
        }
    }
}

fn listado_productos(p: &Path) {
    let mut temp: String = String::from("Los productos del kiosko caibi son:\n");
    if let Ok(lines) = leer(p) {
        for line in lines {
            let linea: String = line.unwrap();
            let mut item: [&str; 3] = [""; 3];
            for (i, elem) in linea.split(",").enumerate() {
                item[i] = elem;
            }
            println!("{}: ${} stock: {}", item[0], item[1], item[2]);
        }
    }
}

fn mostrar_reserva(p: &Path) {
    println!("esta decision es valido solo para el personal de negocion, por favor ingrese el codigo maestro ");
    let mut clave: String = input();
    clave = clave.to_lowercase();
    while clave == "icb" {
    let mut temp: String = String::from("Las reservas del kiosko caibi son:\n");
    if let Ok(lines) = leer(p) {
        for line in lines {
            let linea: String = line.unwrap();
            let mut item: [&str; 5] = [""; 5];
            for (i, elem) in linea.split(",").enumerate() {
                item[i] = elem;
            }
            println!("");
            println!("nombre: {}, producto: {}, hora: {}, dia: {}, cantidad: {}", item[0], item[1], item[2], item[3], item[4]);
            println!("");
        }
    }
    break;
    } 
}

fn listado_horarios(p: &Path) {
    let mut temp: String = String::from("Los horarios del kiosko caibi son:\n");
    if let Ok(lines) = leer(p) {
        for line in lines {
            let linea: String = line.unwrap();
            let mut item: [&str; 3] = [""; 3];
            for (i, elem) in linea.split(",").enumerate() {
                item[i] = elem;
            }
            println!("{}: inicio {}, termino {}", item[0], item[1], item[2]);
        }
        println!("");
    }
}
fn identificar(dato: String) -> usize {
    let mut index: usize = 0;
    let dato:u32 = dato.parse().unwrap();
    index = (usize::try_from(dato).unwrap()) - 1;
    return index;
}

fn modificar_producto(p: &Path) {
    println!("esta decision es valido solo para el personal de negocion, por favor ingrese el codigo maestro ");
    let mut clave: String = input();
    clave = clave.to_lowercase();
    while clave == "icb" {
        listado_productos(p);
        println!("Ingrese el nombre del Producto:");
        let mut nombre: String = input();
        nombre = nombre.to_lowercase();
        let mut cont: u8 = 0;
        let mut temp: String = String::new();
        if let Ok(lines) = leer(p) {
            for line in lines {
                let linea: String = line.unwrap();
                let mut item: [&str; 3] = [""; 3];
                for (i, elem) in linea.split(",").enumerate() {
                    item[i] = elem;
                }
                if nombre == item[0] {
                    let mut temp2: String = String::new();
                    let dato: String = dec2(); // 1 para nombre, 2 para precio, 3 para stock
                    println!("Ingrese la modificación:");
                    let mut modi: String = input();
                    modi = modi.to_lowercase();
                    let index: usize = identificar(dato);
                    item[index] = &modi;
                    for i in 0..3 {
                        if i == 2 {
                            temp2 = format!("{}{}", temp2, item[i]); 
                            println!("{}", temp2); 
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
                println!("");
                println!("Este Producto no se encuentra en la base de datos.");
                println!("");
            } else {
                modificar_archivo(p, temp);
                println!("");
                println!("La base de datos a sido modificada con éxito.");
                println!("");
                break;
            }
        } else {
            println!("");
            println!("Ocurrio un error al ingresar al archivo");
            println!("");
        }
    }
}
//modificar este funcion(recordatorio)
fn dec2() -> String {
    println!(
        "Ingrese el dato que desea modificar:\n1: Nombre del producto.\n2: Precio de venta.\n3: stock.\n"
    );
    let mut dato: String = input();
    while dato != "1" && dato != "2" && dato != "3" {
        println!("Por favor ingrese un número entre 1 y 3");
        dato = input();
    }
    return dato;
}

fn eliminar_producto(p: &Path) {
    println!("esta decision es valido solo para el personal de negocion, por favor ingrese el codigo maestro ");
    let mut clave: String = input();
    clave = clave.to_lowercase();
    while clave == "icb" {
        println!("Ingrese un producto que desea eliminar:");
        let mut nombre: String = input();
        nombre = nombre.to_lowercase();
        let mut cont: u8 = 0;
        let mut temp: String = String::new();

        if let Ok(lines) = leer(p) {
            for line in lines {
                let linea: String = line.unwrap();
                let mut item: [&str; 3] = [""; 3];
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
            println!("");
            println!("Este producto no se encuentra en la base de datos.");
            println!("");
        } else {
            modificar_archivo(p, temp);
            println!("");
            println!("La base de datos a sido modificada con éxito.");
            println!("");
        }
        break;
    }
}

fn borrar_reserva(){
    let path_reserva = Path::new("reservas.txt");
    println!("esta decision es valido solo para el personal de negocion, por favor ingrese el codigo maestro ");
    let mut clave: String = input();
    clave = clave.to_lowercase();
    while clave == "icb" {
        let mut cont: u8 = 1;
        let mut temp: String = String::from(" ");
        modificar_archivo(path_reserva, temp);
        println!("");
        println!("La base de datos a sido borrada con éxito.");
        println!("");
        break;
    }

}



fn agregar_productos(p: &Path) {
    println!("esta decision es valido solo para el personal de negocion, por favor ingrese el codigo maestro ");
    let mut clave: String = input();
    clave = clave.to_lowercase();
    while clave == "icb" {
        let path = Path::new("kiosko_caibi.txt");
        println!("Ingrese nombre del nuevo producto:");
        let mut nombre: String = input();
        nombre = nombre.to_lowercase();

        println!("Ingrese precio del nuevo producto:");
        let mut precio: String = input();
        precio = precio.to_lowercase();

        println!("Ingrese stock del nuevo producto:");
        let mut stock: String = input();
        stock = stock.to_lowercase();

        let m = nuevo_producto(nombre, precio, stock);
        let temp = format!("{},{},{}\n", m.nombre, m.precio, m.stock,);
        let mut file = agregar_archivo(path);
        file.write_all(temp.as_bytes());
        println!("");
        println!("ha sido añadido con éxito.");
        println!("");
        break;
    }
}

fn menu() -> String {
    println!(
        "Bienvenido al Kiosko Caibi, por favor seleccione una de las siguientes opciones:
                          ----Para salir del menu presione la tecla x----
\x1b[93m 1: Ingresar una reserva.\x1b[0m
\x1b[93m 2: Consultar el precio de un producto.\x1b[0m
\x1b[93m 3: Listar los productos del kiosco caibi.\x1b[0m
\x1b[32m 4: Modificar un elemento de un producto.\x1b[0m
\x1b[31m 5: Eliminar un producto de kiosco caibi.\x1b[0m
\x1b[32m 6: Agregar nuevos productos al kiosco caibi.\x1b[0m
\x1b[32m 7: Ver horario (semanal) del kiosco caibi.\x1b[0m
\x1b[32m 8: Ver reservas del kiosco caibi.\x1b[0m
\x1b[32m 9: borrar reservas del kiosco caibi.\x1b[0m"
    );
    let mut decision: String = input();
    while decision != "1"
        && decision != "2"
        && decision != "3"
        && decision != "4"
        && decision != "5"
        && decision != "6"
        && decision != "7"
        && decision != "x"
        && decision != "8"
        && decision != "9"
    {
        println!("Por favor ingrese un número entre 1 y 9");
        decision = input();
    }
    return decision;
}

fn camino(path: &Path, decision: &str) {
    if decision == "1" {
        reservar();
        main();
    } else if decision == "2" {
        consulta_precio(path);
        main();
    } else if decision == "3" {
        listado_productos(path);
        main();
    } else if decision == "4" {
        modificar_producto(path);
        main();
    } else if decision == "5" {
        eliminar_producto(path);
        main();
    } else if decision == "6" {
        agregar_productos(path);
        main();
    } else if decision == "7" {
        let path_horario = Path::new("horario.txt");
        listado_horarios(path_horario);
        main();
    } else if decision == "8" {
        let path_reservas = Path::new("reservas.txt");
        mostrar_reserva(path_reservas);
        main();
    } else if decision == "x"{
        process::exit(1);
    }else if decision == "9"{
        borrar_reserva();
        main();
    }

}

fn main() {
    let path = Path::new("kiosko_caibi.txt");
    let decision: String = menu();
    camino(path, &decision);
    
}

