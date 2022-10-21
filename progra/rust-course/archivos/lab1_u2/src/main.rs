use std::fs::File;
use std::path::Path;
use std::io::Read;
use std::fs::OpenOptions;
use std::io::Write;

fn obtener_menos_popular(canciones: &[&str]){
    let mut menos_popular: Vec<&str> = Vec::new();
    let mut min_rank = 100;
    for cancion in canciones.iter(){
        let datos_cancion: Vec<&str> = (*cancion).split(",").collect();
        let num_elementos = datos_cancion.len();
        let rank_cancion = datos_cancion[num_elementos-1];
        let rank_cancion = rank_cancion.parse::<i32>().unwrap();
        if rank_cancion < min_rank{
            min_rank = rank_cancion;
            menos_popular = datos_cancion;
        }
    }
    println!("cancion menos popular: {}, rango: {}, num: {}", menos_popular[1],min_rank, menos_popular[0])
}

fn obtener_mas_popular(canciones: &[&str]){
    let mut mas_popular: Vec<&str> = Vec::new();
    let mut max_rank = 0;
    for cancion in canciones.iter(){
        let datos_cancion: Vec<&str> = (*cancion).split(",").collect();
        let num_elementos = datos_cancion.len();
        let rank_cancion = datos_cancion[num_elementos-1];
        let rank_cancion = rank_cancion.parse::<i32>().unwrap();
        if rank_cancion > max_rank{
            max_rank = rank_cancion;
            mas_popular = datos_cancion;
        }
    }
    println!("cancion mas popular: {}, rango: {}, num: {}", mas_popular[1],max_rank, mas_popular[0])
}

fn procesar_canciones(text: String){
    let lineas_separadas: Vec<&str> = text.split("\n").collect(); // el ultimo elemento esta vacio
                                                                  // ->""
    let _nombres_columnas = lineas_separadas[0];
    let lineas_separadas = &lineas_separadas[1..&lineas_separadas.len()-1]; // lo quito
    obtener_mas_popular(lineas_separadas);
    obtener_menos_popular(lineas_separadas);
}



fn read_file(mut f: &File){
    let mut text = String::new();
    f.read_to_string(&mut text).unwrap();
    //println!("{}", &text);

    procesar_canciones(text);
    //return text;
}

fn create_blank_file(p: &Path){
    let file = File::create(p).expect("El archivo no pudo crearse");
    println!("El archivo fue creado");

}


fn add_new_content(mut f: &File){
    let nueva_cancion = "51,dakiti,bad bunny,jhay cortez,reggaeton,95\n";
    let nueva_mas_popular = "52,yonaguni,bad bunny,urbano latino,97\n";
    let mut nuevas_lineas = nueva_cancion.to_string();
    nuevas_lineas += nueva_mas_popular;

    f.write_all(nuevas_lineas.as_bytes());
}

fn open_file_to_append(p: &Path) -> File{

    let mut binding = OpenOptions::new();
    let binding = binding.append(true);
    let file = match binding.open(p){
        Err(why) => panic!("No se puede abrir el archivo"),
        Ok(file) => file,
    };

    //add_new_content(&file);

    return file

}

fn open_file_to_read(p: &Path){
    if Path::new(p).exists(){
        let file = match File::open(&p){
            Err(why) => panic!("El archivo no se puede abrir..."),
            Ok(file) => file,
        };
        read_file(&file);
    } else {
        create_blank_file(p);
    }
}


fn main(){
    let path = Path::new("./top50.csv");
    //let new_path = Path::new("./top50.csv");
    open_file_to_read(path);
    let file = open_file_to_append(path);
    //let file = open_file_to_append(path);
    add_new_content(&file);
    println!("despues de añadir");
    open_file_to_read(path);
}
