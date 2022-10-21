// The syntax of lifetime annotation:
// let x: &i32
// let x: &'a i32
// let x: &'a mut i32

/** For some lifetime 'a, the function "longest" will take two references that
 * are valid within the lifetime 'a and will return a reference that is valid
 * within the lifetime 'a
 */
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() { return x; }
    else { return y; }
}

fn test_longest() {
    let msg1 = "Hello, world";
    {
        /* When filling in concrete variables, the generic lifetime is the
         * overlap between the two arguments, so the returned reference is
         * valid if and only if the other two variables are also valid
         */
        let msg2 = String::from("foobar");
        let longest_msg = longest(msg1, &msg2);
        println!("{}", longest_msg);
    }
    // msg2 is out of scope, so longest_msg is no longer valid
    // println!("{}", longest_msg);
}

/** Using lifetime annotation on struct definition specifies the relationship
 * between a struct's lifetime and its field references' lifetime.
 * The definition below says that the struct cannot outlive the lifetime of
 * the field "content"
 */
struct Excerpt<'a> {
    content: &'a str,
}

/** When lifetime annotation is deterministic and predictable, the programmer
 * doesn't have to provide explicit lifetime annotation
 */
fn firstword(msg: &str) -> &str {  // there is no other lifetime possible
    let bytes = msg.as_bytes();

    for (i, &byte) in bytes.iter().enumerate() {
        if byte == b' ' {
            return &msg[..i];
        }
    }

    return &msg
}

