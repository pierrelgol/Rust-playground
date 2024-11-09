use rand::Rng;
use std::io;

fn main() {
    println!("Guess the number!");
    println!("Please input your guess.");

    let secret_number: u8 = rand::thread_rng().gen_range(1..=100);

    println!("the current secret_number is {secret_number}");

    let mut guess = String::new();
    let mut cond = false;

    loop {
        if cond != false {
            break;
        }

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let value: u8 = guess
            .trim()
            .parse::<u8>()
            .expect("Error while parsing the number.");

        if value == secret_number {
            println!("You'r guess {guess} was right you found the secret number {secret_number}!");
            cond = true;
        } else {
            println!("You'r guess {guess} was wrong!\n try again!");
        }
    }
}
