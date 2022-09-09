

fn is_armstrong_number(num: u32) -> bool{
    let mut sum = 0;
    let number = num.to_string();
    for digit in number.chars() {
        let digit = digit as u32 - '0' as u32;
        sum += digit.pow(number.len() as u32);
        
    }
    
    if sum == num {
        return true;
    }else {
        return false;
    }
}

fn main() {
    
    let mut contador: i32 = 0;

    for i in 0..1000 {
        if is_armstrong_number(i) {
            contador += 1;
            println!("{}",i);
        } 
    }

    print!("existen {} numeros de amrstrong entre 0 y 1000", contador);
}
