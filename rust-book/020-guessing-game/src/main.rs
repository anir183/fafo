use std::{cmp::Ordering, io};

fn main() {
    let secret_number: u8 = rand::random_range(0..=100);

    loop {
        println!("Guess a number [0 - 100]:");

        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read input!");

        println!();

        if guess.trim() == "quit" {
            println!("Quitting!");
            println!();
            break;
        }

        let guess: u8 = match guess.trim().parse() {
            Ok(num) => {
                if num > 100 {
                    println!("Number greater than 100!");
                    println!();
                    continue;
                } else {
                    num
                }
            }
            Err(_) => {
                println!("Invalid positive integer!");
                println!();
                continue;
            }
        };

        println!("You guessed: {guess}");

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }

        println!();
    }
}
