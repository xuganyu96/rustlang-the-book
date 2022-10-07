/**
 * First let's build a function that returns the index of the first instance
 * of the input characther in the input string. If the input character does
 * not exist, return -1
 */
fn strchr(s: &String, c: u8) -> usize {
    let bytes: &[u8] = s.as_bytes();

    for (i, &byte) in bytes.iter().enumerate() {
        if byte == c {
            return i;
        }
    }

    return 0;
}

fn main() {
    let msg: String = String::from("Hello, world");
    let chr: u8 = b' ';
    println!("{}", strchr(&msg, chr));
}
