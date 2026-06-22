use rand::RngExt;
use std::io;
use std::num::ParseIntError;

fn main() -> Result<(), &'static str> {
    let mut rng = rand::rng();
    let random_num: i32 = rng.random_range(0..11);
    let mut tries: i32 = 0;
    println!("Guess the number! and input -1 to stop the game.");
    while true {
        let guess: i32 = match take_input() {
            Ok(number) => number,
            Err(_) => return Err("Please Enter a Correct Number"),
        };
        if guess == -1 {
            break;
        }
        tries += 1;
        if guess == random_num {
            println!("YOU GUESSED IT RIGHT!!!!!!!!!!!!!!!!!");
            break;
        } else if guess < random_num {
            println!("GUESS HIGHER YOU BITCH");
        } else {
            println!("GUESS LOWER YOU BITCH");
        }
    }
    println!("YOU TOOK {} TRIES TO WIN",tries);
    Ok(())
}

fn take_input() -> Result<i32, ParseIntError> {
    println!("Please input your guess");

    let mut guess: String = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

    let number = guess.trim().parse::<i32>()?;

    Ok(number)
}
