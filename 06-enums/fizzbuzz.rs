fn fizzbuzz(x: i32) {
    match (x % 3, x % 5) {
        (0, 0) => println!("fizzbuzz"),
        (0, _) => println!("fizz"),
        (_, 0) => println!("buzz"),
        _ => (), // the catch-all pattern can also use the "other" keyword
                 // if you need the value; otherwise, use the placeholder "_"
    }
}

fn main() {
    fizzbuzz(1);
    fizzbuzz(9);
    fizzbuzz(10);
    fizzbuzz(15);
}
