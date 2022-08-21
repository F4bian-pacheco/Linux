
// estos son los comentarios de una linea

/*Pero estos
son los comentarios
multilinea*/


fn main(){
    println!("Hello World");
    println!("I'm a Rustacean");

    // Formatted print

    let x = 5 + /* 90 + */ 5;
    println!("Is `x` 10 or 100? x = {}", x);

    println!("Hola mi nombre es {1} {0}", "Pacheco", "Fabian");

    println!("Base 10 repr:               {}",   69420);
    println!("Base 2 (binary) repr:       {:b}", 69420);
    println!("Base 8 (octal) repr:        {:o}", 69420);
    println!("Base 16 (hexadecimal) repr: {:x}", 69420);
    println!("Base 16 (hexadecimal) repr: {:X}", 69420);

    let pi = 3.141592;
    println!("Pi is roughly {pi:.3}");

}
