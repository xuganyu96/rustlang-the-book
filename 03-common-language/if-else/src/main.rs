/**
 * Classic fizzbuzz: if divisible by 3, print "fizz"; if divisble by 5, print
 * "buss"; if divisible by both 3 and 5, print "fizzbuzz"
 */
fn fizzbuzz(x: i32) {
    if x % (3 * 5) == 0 {
        // Note 1: parenthesis is not required in "if" statement. Rust
        //   compiler will complain if extraneous parentheses are used
        // Note 2: Rust does not work with "truthy" and/or "falsy" values;
        //   "if" statements must use boolean values
        println!("fizzbuzz");
    } else if x % 3 == 0 {
        println!("fizz");
    } else if x % 5 == 0 {
        println!("buzz");
    }
}

fn ternaryfizzbuzz(x: i32) {
    let three: bool = x % 3 == 0;
    let five: bool = x % 5 == 0;

    let toprint: &str = if three && five {
        "fizzbuzz"
    } else if three {
        "fizz"
    } else if five {
        "buzz"
    } else {
        ""
    };
    println!("{toprint}");
}

fn main() {
    ternaryfizzbuzz(15);
    fizzbuzz(15);
}

