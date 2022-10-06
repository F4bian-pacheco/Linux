

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
        let mut entrada = String::new();
        println!("Ingrese el valor {} de su arreglo",i+1);
        let _b = std::io::stdin().read_line(&mut entrada).unwrap();
        entrada = entrada.trim().to_string();
        let entrada_int = entrada.parse::<i32>().unwrap();
        arreglo[i] = entrada_int;
    }
    return arreglo;
}

fn main() {
    println!("Arreglo 1");
    let mut tamaño_n = String::new();
    let mut tamaño_m = String::new();
    println!("Ingrese el tamaño del arreglo 1: ");
    let _b = std::io::stdin().read_line(&mut tamaño_n).unwrap();
    tamaño_n = tamaño_n.trim().to_string();
    let tamaño_n = tamaño_n.parse::<usize>().unwrap();

    let arreglo_a = obtener_arreglo(tamaño_n);

    println!("Arreglo 2");
    println!("Ingrese el tamaño del arreglo 2: ");
    let _b = std::io::stdin().read_line(&mut tamaño_m).unwrap();
    tamaño_m = tamaño_m.trim().to_string();
    let tamaño_m = tamaño_m.parse::<usize>().unwrap();

    let arreglo_b = obtener_arreglo(tamaño_m);
    println!("{:?}",arreglo_a);
    println!("{:?}",arreglo_b);
    let c = combinar_arreglos(arreglo_a,arreglo_b);
    println!("{:?}",c);
}
