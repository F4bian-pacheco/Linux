

fn recorrer_por_columnas(arreglo :Vec<Vec<i32>>){
    for i in 0..arreglo.len() {
        for j in 0..arreglo[i].len(){
            print!("{},", arreglo[j][i]);
        }
    }
    println!("");
}

fn imprimir_matriz(matriz: Vec<Vec<i32>>){
    for fila in matriz {
        println!("{:?}",fila);
    }
}

fn main() {
    let filas = 4;
    let cols = 4;
    let mut arreglo_bi :Vec<Vec<i32>> = vec![vec![0;filas];cols];
    for i in 0..arreglo_bi.len() {
        for j in 0..arreglo_bi[i].len() {
            println!("Ingrese valor posicion [{}][{}]",i,j);
            let mut entrada = String::new();
            let _b = std::io::stdin().read_line(&mut entrada).unwrap();
            entrada = entrada.trim().to_string();
            let entrada = entrada.parse::<i32>().unwrap();
            arreglo_bi[i][j] = entrada;
        }
    }
    imprimir_matriz(arreglo_bi.clone());
    recorrer_por_columnas(arreglo_bi.clone());
    
}
