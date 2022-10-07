
fn obtener_volumen(ancho :i32, largo:i32, alto:i32) -> i32 {
    let volumen = ancho*largo*alto;
    return volumen
}

fn ordenar_cajas(cajas :&mut Vec<i32>){
    
    for i in 0..cajas.len() {
        for j in 0..cajas.len()-1-i{
            if cajas[j] > cajas[j+1] {
                let temp = cajas[j];
                cajas[j] = cajas[j+1];
                cajas[j+1] = temp;
            }
        }
    }
}

fn main() {
    
    let mut cajas :Vec<i32> = vec![0i32;10];

    for indice in 0..10 {
        
        let mut ancho = String::new();
        let mut largo = String::new();
        let mut alto = String::new();

        println!("Ingrese el ancho de la caja {}: ",indice+1);
        let _b = std::io::stdin().read_line(&mut ancho).unwrap();
        ancho = ancho.trim().to_string();
        let ancho = ancho.parse::<i32>().unwrap();
        
        println!("Ingrese el largo de la caja {}: ",indice+1);
        let _b = std::io::stdin().read_line(&mut largo).unwrap();
        largo = largo.trim().to_string();
        let largo = largo.parse::<i32>().unwrap();
        
        println!("Ingrese el alto de la caja {}: ",indice+1);
        let _b = std::io::stdin().read_line(&mut alto).unwrap();
        alto = alto.trim().to_string();
        let alto = alto.parse::<i32>().unwrap();

        let volumen = obtener_volumen(ancho,largo,alto);
        cajas[indice] = volumen;
        
    }
    
    println!("cajas antes: {:?}", cajas);
    ordenar_cajas(&mut cajas);
    println!("cajas despues: {:?}", cajas);

}
