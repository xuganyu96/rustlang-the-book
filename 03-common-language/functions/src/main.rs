/**
 * 4.1: functions
 * Functions signature requires explicit type declaration for all parameters
 * and return type.
 *
 * Return value can be explicitly specified using the "return" statement, but
 * can also be implicitly return using the last expression. If using "last
 * expression" to return, make sure that the last expression is not a
 * statement with a semicolon after it
 */
fn main() {
    let x: i32 = 2147483646;
    let inc: i32 = increment(x);
    let dec: i32 = decrement(x);
    println!("{x} + 1 is {inc}");
    println!("{x} - 1 is {dec}");
}

/**
 * interestingly enough, overflow is not repoerted at compile time (which is
 * understandable); however, overflow will cause error at runtime:
 * "thread 'main' panicked at 'attempt to add with overflow'"
 */
fn increment(x: i32) -> i32 {
    return x + 1; 
}

/**
 * Rust functions can also return values by having an expression at the end
 */
fn decrement(x: i32) -> i32 {
    // in fact, any code block can evaluate to the last expression in itself
    let y: i32 = {
        x - 1  // code block evals to this expression
    };
    y  // function returns this expression. DO NOT INCLUDE SEMICOLON
}

