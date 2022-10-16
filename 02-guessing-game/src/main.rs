use std::cmp::Ordering;
use std::io;
use rand::Rng;

fn main() {
    let secret = rand::thread_rng().gen_range(1..=100);
    println!("Guess the number!");

    println!("Please input your guess.");


    loop {
        let mut guess = String::new();
        io::stdin()  // returns a stdin handle
            .read_line(&mut guess)  // returns a "Result" object
            .expect("Failed to read line");  // handles result being "Err"
        let guess: i32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Please inpug valid number");
                continue
            }
        };

        match guess.cmp(&secret) {
            Ordering::Greater => println!("Guess is too big"),
            Ordering::Less => println!("Guess is too small"),
            Ordering::Equal => {
                println!("Guess is correct");
                break;
            }
        }
    }
}
