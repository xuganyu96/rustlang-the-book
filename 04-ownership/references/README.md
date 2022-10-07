# References
Rust's reference is the first "tricky" concept to grasp. Here is some note to clarify some points that are not directly mentioned in the book.

## Accessing fields/methods
In C, the syntax for accessing a field is different between values and references. In rust, the syntax is the same between the value itself and immutable reference to a value, although you cannot call functions that will mutate the data on an immutable reference.

```rust
let hello: String = String::from("Hello, world");
let hello_ref: &String = &hello;

println!("{}, {}", hello.len(), hello_ref.len());  // both works
```

```rust
let mut msg: String = String::from("foo");
let msgref: &mut String = &mut msg;
msgref.push_str("bar");

println!("{}", msg);  // this also works
```

## Mutable reference's exclusivity
There cannot be other references (mutable or not) to a value when there is a mutable reference that will be used later. Here are a few examples:

```rust
/**
 * this is okay because by the time the immutable reference is created, the
 * mutable reference is already done
 */
let mut msg: String = String::from("foo");

let mutable_ref: &mut String = &mut msg;
println!("{}", mutable_ref);

let immutable_ref: &String = &msg;
println!("{}", immutable_ref);
```

```rust
/**
 * this is also okay because by the time the mutable reference is created, the
 * immutable reference is already done
 */
let mut msg: String = String::from("foo");

let immutable_ref: &String = &msg;
println!("{}", immutable_ref);

let mutable_ref: &mut String = &mut msg;
println!("{}", mutable_ref);
```

```rust
/**
 * This is not okay. The immutable reference is used after the the mutable
 * reference is created, which means that after the mutable reference is
 * created, there are two references to the same value
 */
let mut msg: String = String::from("foo");
                                           
let immutable_ref: &String = &msg;
                                           
let mutable_ref: &mut String = &mut msg;
println!("{}", mutable_ref);
                                           
println!("{}", immutable_ref);
```
