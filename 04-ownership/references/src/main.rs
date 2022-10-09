fn main() {
    let msg: String = String::from("Hello, world"); // msg owns the value
    let new: String = memmove(msg);

    // println!("{msg}"); // will not compile: data freed after memmove exits
    println!("{new}");

    let newlen: usize = strlen(&new);
    println!("{newlen}");

    let mut msg: String = String::from("foo");
    append_str(&mut msg, "bar");
    println!("{msg}");
    let msg_ref2: &String = &msg;
    println!("{msg_ref2}");
    // a mutable reference itself is not necessarily mutable
    let msg_ref: &mut String = &mut msg;
    // let msg_ref2: &String = &msg;  // will not compile
    println!("{msg_ref}");

    let msg = dangle();
    println!("{msg}");
}

/**
 * When a variable that owns a piece of data is reassigned to another variable
 * the ownership is transferred to the new variable. This also applies to when
 * values to passed into functions and when values are returned from functions
 */
fn memmove(_from: String) -> String {
    let to: String = String::from("Hello, world");
    return to;
}

/**
 * An immutable reference to a value can be arbitrarily created. They are safe
 * because they are read-only
 */
fn strlen(strref: &String) -> usize {
    return strref.len()  // references can be freed and not affect the owner
}

/**
 * A mutable reference to a value can mutate the data underneath. Mutable
 * reference can only be made for mutable variables. No other reference,
 * mutable or not, can be made when there is a mutable reference in scope
 */
fn append_str(s: &mut String, t: &str) {
    s.push_str(t);
}

/**
 * Rust will not allow functions to return references to data that are already
 * freed, such as when the owner goes out of scope
 */
fn dangle() -> String {
    let msg: String = String::from("Hello, world");
    // return &msg;  // will not compile since the function returned dangling
                     // pointer to freed memory 
    return msg;  // this is okay: the ownership will be transferred
}

