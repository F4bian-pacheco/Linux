

fn combinar_arreglos(arr1: Vec<i32>, arr2: Vec<i32>) -> Vec<i32>{
    let _largo_arr1 = arr1.len();
    let _largo_arr2 = arr2.len();

    let mut arr_combinado = [arr1,arr2].concat();
    arr_combinado.sort();
    arr_combinado.reverse();
    return arr_combinado;
}

fn obtener_arreglo(n :usize) -> Vec<i32> {
    let mut arreglo = vec![0;n];

    for i in 0..n{
        arreglo[i] = (i*n) as i32;
    }
    return arreglo;
}

fn main() {
    println!("Hello, world!");
    let a = obtener_arreglo(3);
    let b = obtener_arreglo(5);
    println!("{:?}",a);
    println!("{:?}",b);
    let c = combinar_arreglos(a,b);
    println!("{:?}",c);
}
