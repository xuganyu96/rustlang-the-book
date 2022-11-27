use std::env::args;

fn main() {
    let secret = 10;
    let mut args = args();
    args.next(); // binary path
    let guess = args.next().unwrap();
    let guess: Result<i32, _> = guess.parse();

    match guess {
        Ok(guess) if guess == secret => println!("Matched secret"),
        _ => println!("Did not match secret"),
    }
}
