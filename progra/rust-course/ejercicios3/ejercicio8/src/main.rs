fn main() {
    println!("Ingrese una fecha con el siguiente formato(dd/mm): ");
    let mut fecha  = String::new();
    let  _b2 = std::io::stdin().read_line(&mut fecha).unwrap();
    
    let fecha = fecha.trim();
                                    
    let meses = vec!["enero","febrero","marzo","abril","mayo","junio",
                                                    "julio","agosto","septiembre","octubre",
                                                    "noviembre","diciembre"];
    
    let fecha: Vec<&str> = fecha.split("/").collect();

    let dia = fecha[0].to_string().parse::<i32>().unwrap();
    let mes = fecha[1].to_string().parse::<usize>().unwrap();

    if dia > 28 && meses[mes] == "febrero" {
        println!("Febrero tiene solo 28 dias, ingrese una fecha correcta");
    }

    println!("{} de {}",dia,meses[mes-1]);
                                                
}
