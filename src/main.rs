use rand::Rng;
use std::cmp::Ordering;
use std::io;
fn main() {
    println!("Adivina el numero!");

    let secret_number = rand::thread_rng().gen_range(1..=100);

    loop {
        println!("Ingresa un numero.");

        let mut guess = String::new();

        io::stdin().read_line(&mut guess).expect("Fallo para leer la linea");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                continue;
            }
        };

        match guess.cmp(&secret_number) {
            Ordering::Less => print!("Muy bajo"),
            Ordering::Greater => print!("Muy alto"),
            Ordering::Equal => {
                print!("Has adivinado!");
                break;
            }
        }
    }
}
