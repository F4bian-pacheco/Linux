fn main() {
    println!("Ingrese una hora con el siguiente formato(hh:mm:ss): ");
    let mut tiempo  = String::new();
    let  _b2 = std::io::stdin().read_line(&mut tiempo).unwrap();
    
    let tiempo = tiempo.trim();

    // let tiempo = "12:34:56";

    let dates: Vec<&str> = tiempo.split(":").collect();

    let mut hora = dates[0].to_string().parse::<i32>().unwrap();
    let mut minutos = dates[1].to_string().parse::<i32>().unwrap();
    let mut segundos = dates[2].to_string().parse::<i32>().unwrap();

    let new_minutos = 1;
    let new_segundos = 1;
    
    minutos += new_minutos;
    segundos += new_segundos;

    println!("{}:{}:{}",hora,minutos,segundos);


}
