use std::io::stdin;

fn main(){
    let mut adn = String::new();
    stdin().read_line(&mut adn).unwrap();
    
    let adn = adn.trim().to_uppercase();
    let mut arn = String::new();
    for nucleotido in adn.chars() {
        if nucleotido == 'G' {
            arn += "C"
        }
        if nucleotido == 'C'{
           arn += "G" 
        }
        if nucleotido == 'T' {
            arn += "A"
        }
        if nucleotido == 'A' {
            arn += "U"
        }
        
    }
    println!("ADN: {}", adn);
    println!("trasncripcion ARN: {}", arn);
}
