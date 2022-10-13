// Option, Some, and None are all keywords that already exist; no need to
// explicitly bring them into scope
// enum Option<T> {
//     None,
//     Some(T)
// }

fn increment(x: Option<i32>) -> Option<i32> {
    match x {
        None => {
            println!("Will return None");
            None
        },
        Some(v) => {
            println!("Will return {}", v + 1);
            Some(v + 1)
        },
    }
}

/** the "if let Some(v) = ..." syntax can also be used to extract value out of
 * an enum variant that carries data. This is good for if we only need to
 * handle a single case
 */
fn iflet(x: Option<i32>) -> Option<i32> {
    if let Some(v) = x {
        println!("Will return {}", v + 1);
        return Some(v + 1);
    }
    return None;
}

fn main() {
    let x: Option<i32> = Some(0);
    let none: Option<i32> = None;

    increment(x);
    increment(none);
}
