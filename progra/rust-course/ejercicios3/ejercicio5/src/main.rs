fn main() {

    let mut sueldo_base = String::new();
    println!("Ingrese su sueldo base: ");
    let  _b1 = std::io::stdin().read_line(&mut sueldo_base).unwrap();
    sueldo_base = sueldo_base.trim().to_string();
    let sueldo_base = sueldo_base.parse::<f32>().unwrap();


    let mut num_ventas = String::new();
    println!("Ingrese el numero de ventas: ");
    let _b1 = std::io::stdin().read_line(&mut num_ventas).unwrap();
    num_ventas = num_ventas.trim().to_string();
    let num_ventas = num_ventas.parse::<i32>().unwrap();


    let mut _total_ventas: f32 = 0.0;
    let mut total_comisiones: f32 = 0.0;

    for venta in 0..num_ventas {
        let mut ventas = String::new();
        println!("Ingrese el valor de la venta {}: ",venta+1);
        let  _b1 = std::io::stdin().read_line(&mut ventas).unwrap();
        ventas = ventas.trim().to_string();

        let ventas = ventas.parse::<f32>().unwrap();
        let comision = ventas * 0.07;

        _total_ventas += ventas;
        total_comisiones += comision;
    }

    let sueldo_total = sueldo_base + total_comisiones;

    println!("El sueldo total es: {}", sueldo_total);
    println!("El dinero de comisiones es: {}", total_comisiones);


}
