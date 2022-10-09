/** return the 'length' of the first word in the string */
fn first_word_index(s: &String) -> usize {
    let bytes: &[u8] = s.as_bytes();

    for (i, &byte) in bytes.iter().enumerate() {
        if byte == b' ' {
            return i;
        }
    }

    return s.len();
}

/** return an immutable slice */
fn first_word_slice(s: &String) -> &str {
    for (i, &byte) in s.as_bytes().iter().enumerate() {
        if byte == b' ' { return &s[..i] }
    }
    return &s[..]
}

/**
 * Return a slice of the input string that is the first
 */

fn main() {
    // Get the first word by returning its length`
    let msg: String = String::from("Hello world!");
    let wordlen: usize = first_word_index(&msg);
    let word: &str = &msg[..wordlen];
    println!("{}", word);

    // Get the first word by returning an immutable slice
    println!("{}", first_word_slice(&msg));

    let nums = [0, 1, 2, 3, 4];
    let subarray = &nums[..4];
    assert_eq!(subarray, &[0, 1, 2, 3]);
}

