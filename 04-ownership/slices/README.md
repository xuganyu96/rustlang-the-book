# Notes on slices

## The type of "slice"
One cannot directly create a slice of a String:

```rust
let msg: String = String::from("Hello, word");
let hello: &str = msg[..5]; // will not compile

let msg: &str = "Hello, world";  // the same with String literals
let hello: &str = msg[..5]; // will not compile
let hello: str = msg[..5]; // will not compile
```

```rust
// Slice must be created on a reference
let msg: String = String::from("Hello, world");
let hello: &str = &msg[..5]
```

## Why using immutable slice is safe
Consider the snippet below:

```rust
fn first_word(s: &String) -> &str {
    for (i, &byte) in s.as_bytes().iter().enumerate() {
        if byte == b' ' { return &s[..i] }
    }
    return &s[..]
}

fn main() {
    let mut msg: String = String::from("Hello, world");
    let word: &str = first_word(msg);
    msg.clear()
    println!("{}", word):
}
```

The code above does something sneaky: the variable `word` is a reference to the first word in the string `msg` but then the content of the message is mutated. Rust ensures that this will not produce undefined behavior through the rules of borrowing: **when there is a mutable reference, there cannot be any other reference**. This rule is applied when `msg.clear()` is called, which will attempt to create a mutable slice of the variable `msg`, but `word` is an immutable reference that is still in scope, hence this program will not compile.