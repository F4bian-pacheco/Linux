use std::fmt::{format, Display};
use std::fs::File;
use std::path::Path;
use std::io::Read;
use std::fs::OpenOptions;
use std::io::Write;
use std::vec;


fn obtener_promedio(notas: &[&str]) -> f32{
    let mut total: f32 = 0.0;
    let mut largo = 0;
    for nota in notas.iter(){
        total += (*nota).parse::<f32>().unwrap();
        largo += 1;
    }
    //println!("{}", total/(largo as f32));
    return total/(largo as f32)
    

}


fn procesar_lineas(info: Vec<&str>) -> Vec<Vec<String>>{
    let mut promedios: Vec<Vec<String>> = Vec::new();
    for dato in info{
        if dato.len() > 0{
            let vec_info: Vec<&str> = dato.split(":").collect();
            let mut datos_listos: Vec<String> = Vec::new();
            //println!("{} {:?}",vec_info[0],&vec_info[1..]);
            let promedio = obtener_promedio(&vec_info[1..]);
            let mut promedio_string = String::new();
            if promedio > 4.0{
                promedio_string = "Aprobado".to_string();
            }else{
                promedio_string = "Reprobado".to_string();
            }
            datos_listos.push(vec_info[0].to_string());
            datos_listos.push(promedio_string);
            promedios.push(datos_listos);
            //println!("{:?}",datos_listos);
        }else{
            continue;
        }
    }
    //println!("{:?}",promedios);
    return promedios;
}

fn read_file(mut f: &File) -> Vec<Vec<String>>{
    let mut text = String::new();
    f.read_to_string(&mut text).unwrap();
    let notas_vec: Vec<&str> = (&text).split("\n").collect();
    
    let resultado = procesar_lineas(notas_vec);
    return resultado;
    //println!("{:?}",notas_vec);
    
}

fn create_blank_file(p: &Path){
    let file = File::create(p).expect("El archivo no pudo crearse");
    println!("El archivo fue creado");

}


fn add_new_content(mut f: &File, resultados: Vec<Vec<String>>){
    let mut string_final = String::new();
    for i in resultados{
        let s = format!("{}: {}\n",i[0],i[1]);
        string_final += &s;
    }
    f.write_all(string_final.into_bytes().as_slice());
}

fn add_content(mut f: &File, resultados: Vec<Vec<String>>, display){
    let mut string_final = String::new();
    for i in resultados{
        let s = format!("{}: {}\n",i[0],i[1]);
        string_final += &s;
    }
    file.write_all(string_final.as_bytes()) {
        Err(why) => panic!("couldn't write to {}: {}", display, why),
        Ok() => println!("successfully wrote to {}", display),
    }

}

fn open_file_to_append(p: &Path, resultados: Vec<Vec<String>>) -> File{

    let mut binding = OpenOptions::new();
    let binding = binding.append(true);
    let file = match binding.open(p){
        Err(why) => panic!("No se puede abrir el archivo"),
        Ok(file) => file,
    };

    add_new_content(&file, resultados);

    return file

}

fn write_only_mode(p: &Path, resultados: Vec<Vec<String>>) -> File {
    let display = p.display();
    let mut file = match File::create(&p) {
        Err(why) => panic!("couldn't create {}: {}", display, why),
        Ok(file) => file,
    };
    add_content(&file, resultados, display);

    return file
}

fn open_file_to_read(p: &Path) -> Vec<Vec<String>>{
    if Path::new(p).exists(){
        let file = match File::open(&p){
            Err(why) => panic!("El archivo no se puede abrir..."),
            Ok(file) => file,
        };
        let resultado = read_file(&file);
        return resultado
    } else {
        create_blank_file(p);
        return vec![vec!["no hay alumnos".to_string(), "0.0".to_string()]]
    }
}


fn main(){
    let path = Path::new("./notas.txt");
    let path_resultado = Path::new("./reporte.txt");
    let resultado = open_file_to_read(path);
    println!("{:?}", resultado);
    let file = open_file_to_append(path_resultado, resultado);
    read_file(&file);
}
