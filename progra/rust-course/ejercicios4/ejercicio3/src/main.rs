use rand::Rng;

fn obtener_numeros() -> Vec<i32>{
    let mut arreglo :Vec<i32> = vec![0i32;12];

    for i in 0..12 {
        let num = rand::thread_rng().gen_range(0..101);
        arreglo[i] = num;
    }
    arreglo.sort();
    arreglo.reverse();
    println!("{:?}", arreglo);
    return arreglo;
}

fn mostrar_vidas(vidas: i32){
    println!("Usted perdio {} vidas",vidas);
}

fn obtener_indice(numeros :Vec<i32>, entrada :i32) -> usize {
    let mut indice = 0;
    for i in 0..12 {
        if entrada == numeros[i]{
            indice = i;
            break;
        }     
    }
    return indice;
}

fn verificar(numeros :Vec<i32>, entrada :i32) -> bool{
    
    for i in 0..12 {
        if entrada == numeros[i]{
            return true;
        }
    }
    return false

}

fn iniciar_juego(numeros :Vec<i32>){
    
    let mut vidas = 0;
    let mut lista_escondida :Vec<String> = vec![String::from("*");12];
    loop {
        let mut entrada = String::new();
        
        println!("{:?}",lista_escondida);
        println!("");
        println!("Adivina un numero de la lista entre 0 y 100: ");
        
        let  _b1 = std::io::stdin().read_line(&mut entrada).unwrap();
        entrada = entrada.trim().to_string();        
        let entrada = entrada.parse::<i32>().unwrap();
        
        if entrada < 0 {
            break;
        }
        if verificar(numeros.clone(), entrada) {
            let indice = obtener_indice(numeros.clone(), entrada);
            let num_adivinado = numeros[indice];
            let num_adivinado = num_adivinado.to_string();
            lista_escondida[indice] = num_adivinado;
            
        }else{
            vidas += 1;
        }
    }
    mostrar_vidas(vidas);
}



fn main() {
    println!("Hello, world!");
    let numeros = obtener_numeros();
    iniciar_juego(numeros);
}
