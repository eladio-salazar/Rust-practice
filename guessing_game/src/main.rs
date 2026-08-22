//! Guessing_game main file
//!
//! Proyecto creado mediante cargo para implementar el juego
//! apocaliptico de adivinar un numero del 1-100

use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    
    println!("✨😃✨Guess the number or ☠ die ☠ trying it!✨😃✨");

    let secret_number = rand::thread_rng().gen_range(1..=50);
    let mut tried_counter: u8 = 0;
    const MAX_TRIES: u8 = 4;
    //println!("The secret number is:{secret_number}");

    loop {
        let mut guess = String::new();

        if tried_counter >= MAX_TRIES {
            println!(
                "You have 0 lives left, the secrect number was {} game over.",
                secret_number
            );
            break;
        } else {
            tried_counter += 1;
        }

        println!("🔥🔥🔥¡Input your guest fast!🔥🔥🔥");

        io::stdin()
            .read_line(&mut guess)
            .expect("😱😱Failed to read line, come on.");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("¡Too small🐀!"),
            Ordering::Greater => println!("¡Too big 🐘!"),
            Ordering::Equal => {
                println!("¡You win 🥳!");
                println!("😣You will live 😒, you guessed the number: {guess}​");
                break;
            }
        }
    }
}
