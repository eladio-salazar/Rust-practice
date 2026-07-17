//! Guessing_game main file
//! 
//! Proyecto creado mediante cargo para implementar el juego 
//! apocaliptico de adivinar un numero del 1-100

use std::io;
use rand::Rng;
use std::cmp::Ordering;


fn main() {
    println!("⭐​⭐⭐​​ Guess the number or ☠️​  die☠️​  trying it! ⭐​⭐​⭐​");

    let secret_number = rand::thread_rng().gen_range(1..=100);
    //println!("The secret number is:{secret_number}");

    loop{
        println!("🔥​🔥​🔥​¡Input your guest fast!🔥​🔥​🔥​");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("😱​😱​Failed to read line, come on.");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("🐀​ ¡Too small!"),
            Ordering::Greater => println!("🐘​ ¡Too big!"),
            Ordering::Equal => {
                println!("🥳​ ¡You win!");
                println!("👿​👿​👿​ ​ You will live 😒, you guessed the number: {guess}​");
                break;
            },
        }
    }
}
