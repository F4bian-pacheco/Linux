

fn recorrer_por_columnas(arreglo :Vec<Vec<i32>>){
    for i in 0..arreglo.len() {
        for j in 0..arreglo[i].len(){
            println!("{}", arreglo[j][i]);
        }
        
    }
}



fn main() {
    let filas = 4;
    let cols = 4;
    let mut arreglo_bi :Vec<Vec<i32>> = vec![vec![0;filas];cols];
    for i in 0..arreglo_bi.len() {
        for j in 0..arreglo_bi[i].len() {
            arreglo_bi[i][j] = (i) as i32;
        }
    }
    recorrer_por_columnas(arreglo_bi);
    
}
