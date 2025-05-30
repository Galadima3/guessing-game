use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Welcome to Guess the Number Game!");
    println!("--- Type 'easy' to play on easy mode (1-100)");
    println!("--- Type 'hard' to play on hard mode (1-1000)");

    let secret_number = loop {
        let difficulty = read_input().to_lowercase();
        
        match difficulty.as_str() {
            "easy" => break rand::rng().random_range(1..=100),
            "hard" => break rand::rng().random_range(1..=1000),
            _ => println!("Please enter either 'easy' or 'hard'"),
        }
    };

    let mut guess_count = 0;

    println!("I've chosen a secret number. Can you guess it?");
    
    loop {
        println!("Please input your guess:");

        let guess: u32 = match read_input().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Please type a valid number!");
                continue;
            }
        };

        guess_count += 1;

        println!("You guessed: {guess} (Attempt #{guess_count})");

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("Congratulations! You won in {guess_count} guesses!");
                break;
            }
        }
    }
}

fn read_input() -> String {
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    input.trim().to_string()
}