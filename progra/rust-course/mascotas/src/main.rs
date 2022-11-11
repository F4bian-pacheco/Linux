use std::fs::File;
use std::path::Path;
use std::io::Read;

#[derive(Debug)]
struct Persona{
    nombre: String,
    edad: u8,
    rut: String,
    mascota: Vec<Mascota> // 1) una persona tiene un vector de n mascotas(vector dinamico)
}

#[derive(Debug)]
struct Mascota{
    nombre: String,
    tipo: String,
    color: String,
}

fn obtener_array(texto :&str) -> Vec<&str>{
    let array :Vec<&str> = texto.split("#").collect();
    return array;
}

// 2) recorrer array de personas
fn recorrer_array(personas :Vec<Persona>){
    for persona in personas.iter(){
        println!("
Nombre: {}
Edad: {}
Rut: {}
",persona.nombre, persona.edad, persona.rut);
        for (i,mascota) in persona.mascota.iter().enumerate(){
            println!("
    Mascota {}: 
    Nombre: {}
    Tipo: {}
    Color: {}
",i+1, mascota.nombre, mascota.tipo, mascota.color);
        }
    
    }
}

// 4) Cargar registro de un archivo de texto a una estructura persona
fn read_file(mut f: &File) -> Vec<Persona>{
    let mut text = String::new();
    f.read_to_string(&mut text).unwrap();
    //println!("{:?}", &text);

    let array = obtener_array(&text);
    // 2) crear array de personas
    let mut arr_personas :Vec<Persona> = Vec::new();
    for persona in array{
        let datos_persona :Vec<&str> = persona.trim().split(";").collect();
        let datos_mascotas :Vec<&str> = datos_persona[3].trim().split("\n").collect();
        //println!("{:?}", datos_persona);
        //println!("{:?}", datos_mascotas);

        let edad_int = datos_persona[1].parse::<u8>().unwrap();

        let mut p = Persona{nombre:datos_persona[0].to_string(),
                            edad:edad_int,
                            rut:datos_persona[2].to_string(),
                            mascota:Default::default()};
        for mascota in datos_mascotas{
            let mascota_array :Vec<&str> = mascota.split(",").collect();
            let m = Mascota{nombre:mascota_array[0].to_string(),
                                tipo:mascota_array[1].to_string(),
                                color:mascota_array[2].to_string()
            };
            p.mascota.push(m);
        }

        arr_personas.push(p);
    }

    return arr_personas;
}

fn create_blank_file(p: &Path){
    let _file = File::create(p).expect("El archivo no pudo crearse");
    println!("El archivo fue creado");

}


#[warn(unused_variables)]
fn open_file_to_read(p: &Path) -> Vec<Persona>{
    if Path::new(p).exists(){
        let file = match File::open(&p){
            Err(_why) => panic!("El archivo no se puede abrir..."),
            Ok(file) => file,
        };
        let array_personas = read_file(&file);
        return array_personas;
    } else {
        let arr_vacio : Vec<Persona> = vec![];
        create_blank_file(p);
        return arr_vacio;
    }
}

fn agregar_persona(personas :Vec<Persona>) -> Vec<Persona>{

    let mut aux_array :Vec<Persona> = Vec::from_iter(personas);
    let new_persona = Persona{
        nombre: String::from("Juan"),
        edad: 20,
        rut: String::from("20123432"),
        mascota: vec![Mascota{
            nombre: String::from("rollito"),
            tipo: String::from("erizo"),
            color: String::from("cafe")
        }]
    };
    aux_array.push(new_persona);
    return aux_array;
} 

fn main(){
    let path = Path::new("./data/datos.txt");
    let array_personas :Vec<Persona> = open_file_to_read(path);
    //println!("{:?}",array_personas);

    let array_personas = agregar_persona(array_personas);
    
    recorrer_array(array_personas);


}
