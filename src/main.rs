use rand::{self, Rng};
use std::io;

fn main() {
    let secret= rand::thread_rng().gen_range(1..9999);
    println!("adivinhe o numero! qual seu palpite?");
   
    loop {
   
    let mut palpite = String::new();
    io::stdin().read_line(&mut palpite).expect("erro ao ler ");
    println!("Você disse: {}", palpite);
    
    let palpite=palpite.trim().parse::<i32>().expect("digite apenas numeros!");
    match palpite.cmp(&secret){
        std::cmp::Ordering::Less =>println!("muito baixo"),
        std::cmp::Ordering::Equal =>{ 
            println!("acertou!");
            break;
        },
        std::cmp::Ordering::Greater => println!("muito alto"),
    } 
}
   
}
