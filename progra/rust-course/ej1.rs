use std::io::stdin;

fn is_isbn10(cadena: &str) -> bool{
    // crear y comentar esta función
    let mut resultado: i64 = 0;
    let mut i: i64 = 10;
    for num in cadena.chars() {
        let temp_num: i64 = num.to_string().parse::<i64>().unwrap();
        resultado += temp_num * i;
        i -= 1;
    }
    if resultado % 11 == 0{
        return true
    }else{
        return false
    }
}


fn is_isbn_format_valid(c: &str) -> bool {

    // comentar esta funcion
    if c.chars().next().unwrap().is_numeric() {
        return true;
    } else if c == "X" || c == "x"{
        return true;
    }
    return false
}

fn main(){

    let mut isbn: String = String::new();
    let mut clean_isbn: String = String::new();
    stdin().read_line(&mut isbn).unwrap();
   
    // comentar este ciclo
    for c in isbn.to_string().trim().chars(){
        if is_isbn_format_valid(&c.to_string()){
            clean_isbn = clean_isbn + &c.to_string(); 
        }
    }

    println!("{}", clean_isbn);
    // comentar esta sentencia
    if is_isbn10(&clean_isbn){
        println!("{} es un ISBN10 valido", clean_isbn);
    } else {
        println!("No lo es");
    }

}
