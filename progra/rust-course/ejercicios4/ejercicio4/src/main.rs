

fn mostrar_votos(votaciones :Vec<i32>){
    for (candidato, votos) in votaciones.iter().enumerate() {
        println!("candidato {} : {} votos",candidato+1,votos);
    }
}

fn mostrar_ganador(votaciones :Vec<i32>){
    let mut max = 0;
    let mut max_indice = 0;
    let mut votos_totales = 0;
    for (i,candidato) in votaciones.iter().enumerate(){
        if candidato > &max{
            max = *candidato;
            max_indice = i;
        }
        votos_totales += candidato;
    }
    println!("el candidato ganador es el {} con {} votos",max_indice+1,max);
    println!("con una ponderacion del {}%",(max as f32/votos_totales as f32)*100.0)
}

#[allow(unreachable_code)]
fn votaciones(){
    let mut votaciones_candidatos :Vec<i32> = vec![0i32;25];
 

    loop {
        println!("Ingrese el numero del candidato a votar, para terminar ingrese -1");
        let mut entrada = String::new();
        let _b = std::io::stdin().read_line(&mut entrada).unwrap();
        entrada = entrada.trim().to_string();
        println!("{:?}",entrada);

        if entrada == "-1"{
            break;
        }

        let entrada = entrada.parse::<i32>().unwrap();
        let indice = (entrada as usize)-1;
        votaciones_candidatos[indice] += 1;
     
    }
   
    mostrar_votos(votaciones_candidatos.clone());
    mostrar_ganador(votaciones_candidatos.clone());

}

fn main(){
    votaciones();
   }
