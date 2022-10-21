use std::fs::File;
use std::path::Path;
use std::io::{Read, BufWriter};
use std::io::Write;
use std::vec;

// promedio con este formato
fn obtener_promedio(notas: &[&str]) -> f32{
    let mut total: f32 = 0.0;
    let mut num_notas = 0;
    for nota in notas.iter(){
        total += (*nota).parse::<f32>().unwrap();
        num_notas += 1;
    }
    //println!("{}", total/(largo as f32));
    return total/(num_notas as f32)
    

}

#[warn(unused_assignments)]
fn procesar_lineas(info: Vec<&str>) -> Vec<Vec<String>>{ // info ->
                                                         // ["Alumno1:nota1:not2:...","Alumno2:nota1:.."]
    let mut promedios: Vec<Vec<String>> = Vec::new(); // [["Alumno1","Aprobado"],["Alumno2",Reprobado],...]
    for dato in info{
        if dato.len() > 0{
            let vec_info: Vec<&str> = dato.split(":").collect(); //-> ["Alumno1","4.5","6.4",...]
            let mut datos_listos: Vec<String> = Vec::new();
            let promedio = obtener_promedio(&vec_info[1..]);
            let mut promedio_string = String::new();
            if promedio >= 4.0{
                promedio_string = "Aprobado".to_string();
            }else{
                promedio_string = "Reprobado".to_string();
            }
            datos_listos.push(vec_info[0].to_string()); //->["Alumno1"]
            datos_listos.push(promedio_string); // -> ["Alumno1","Aprobado"]
            promedios.push(datos_listos);
        }else{
            continue;
        }
    }
    return promedios;
}

fn read_file(mut f: &File) -> Vec<Vec<String>>{
    let mut text = String::new();
    f.read_to_string(&mut text).unwrap();
    let notas_vec: Vec<&str> = (&text).split("\n").collect(); // ["Pepito:2.0;...","Joselito:5.6..."]
    
    let resultado = procesar_lineas(notas_vec);
    return resultado;
    //println!("{:?}",notas_vec);
    
}

fn create_blank_file(p: &Path){
    let _file = File::create(p).expect("El archivo no pudo crearse");
    println!("El archivo fue creado");

}


fn open_file_to_read(p: &Path) -> Vec<Vec<String>>{
    if Path::new(p).exists(){
        let file = match File::open(&p){
            Err(_why) => panic!("El archivo no se puede abrir..."),
            Ok(file) => file,
        };
        let resultado = read_file(&file);
        return resultado
    } else {
        create_blank_file(p);
        return vec![vec!["no hay alumnos".to_string(), "0.0".to_string()]]
    }
}

fn get_resultado(notas: Vec<Vec<String>>) -> String{
    let mut string_final = String::new(); // -> "Pepito: Aprobado\nJoselito: Reprobado\n..."
    for i in notas{ // -> ["Alumno1","Aprobado"]
        let s = format!("{}: {}\n",i[0],i[1]);
        string_final += &s;
    }
    
    return string_final;

}


fn main() -> std::io::Result<()>{
    // leer archivo
    let path = Path::new("./notas.txt");
    let resultado = open_file_to_read(path);
    println!("{:?}", resultado);


    let string_final = get_resultado(resultado);

    // escribir en el archivo
     let mut buffer = BufWriter::new(File::create("reporte.txt")?);

     buffer.write_all(string_final.as_bytes())?;
     buffer.flush()?;
     Ok(())
}
