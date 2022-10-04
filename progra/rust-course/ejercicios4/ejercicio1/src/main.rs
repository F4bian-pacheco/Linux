

fn mes_mayor_toneladas(toneladas :[f32;12], meses : [&str;12]) -> &str{

}



fn main() {
    let mut toneladas_por_mes: [f32; 12] = [0.0; 12] ;
    let meses = ["enero", "febrero", "marzo", "abril", "mayo", "junio", "julio", 
                "agosto", "septiembre", "octubre", "noviembre", "diciembre"];
    let mut rng = rand::thread_rng();
    for x in 0..12 {
        let mut toneladas = String::new();
        println!("¿cuantas toneladas de cereal se cosecharon en el mes de {}?: ", meses[x]);
        let  _b1 = std::io::stdin().read_line(&mut toneladas).unwrap();
        toneladas = toneladas.trim().to_string();
        let toneladas = toneladas.parse::<f32>().unwrap();
        toneladas_por_mes[x] = toneladas;
    }
    println!("{:?}", toneladas_por_mes);

}
