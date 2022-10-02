

use rand::Rng;

fn puntaje(puntajes: [i32;10]) -> i32{
    let mut total = 0;
    for punto in puntajes.iter() {
        total += punto;
    }
    return total; 
}

fn print_ganador(puntaje1:i32, puntaje2:i32){
    if puntaje1>puntaje2{
        println!("ganador jugador 1");
    }else {
        println!("ganador jugador 2");
    }
}

fn main() {
    let mut rng = rand::thread_rng();
    
    let jugadores = ["jugador2", "jugador1"];
    let mut puntos_jugador1: [i32; 10] = [0;10];
    let mut puntos_jugador2: [i32; 10] = [0;10];
    let mut indice_j1 = 0;
    let mut indice_j2 = 0;

    let mut pinos = 10;
    let mut rondas = 1;

    
    while rondas <= 20 {
        if rondas%2 == 0{
            println!("turno {}, ronda {}",jugadores[0],rondas);
            let mut total_derribados = 0;
            for lanzamiento in 0..2 {
                let derribados = rng.gen_range(0..pinos);
                pinos = pinos - derribados;
                total_derribados += derribados;
                println!("  lanzamiento {}: {} pinos derribados", lanzamiento+1,derribados);
                if derribados == 10 {
                    break;
                }
            }
            puntos_jugador2[indice_j2] = total_derribados;
            indice_j2 += 1;

        }else {
            
            println!("turno {}, ronda {}",jugadores[1],rondas);
            let mut total_derribados = 0;
            for lanzamiento in 0..2 {
                let derribados = rng.gen_range(0..pinos);
                pinos = pinos - derribados;
                total_derribados += derribados;
                println!("  lanzamiento {}: {} pinos derribados", lanzamiento+1,derribados);
                if derribados == 10 {
                    break;
                }
            }
            puntos_jugador1[indice_j1] = total_derribados;
            indice_j1 += 1;

        }
        println!("pinos restantes: {}\n", pinos);
        rondas += 1;
        pinos = 10;
    }

    println!("ptj jugador1 {:?} -> {}",puntos_jugador1,puntaje(puntos_jugador1));
    println!("ptj jugador2 {:?} -> {}",puntos_jugador2,puntaje(puntos_jugador2));
    
    print_ganador(puntaje(puntos_jugador1),puntaje(puntos_jugador2));
  
}
