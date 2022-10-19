# Common Collections: vectors, strings, hash maps

## Ownership rules of a vector
* If the vector needs to be mutated, then variable that owns the vector must be declared as mutable

```rust
fn fibonacci(n: usize) -> Vec<i32> {
    let mut seq: Vec<i32> = Vec::new();
    for i in 0..n {
        match i {
            0 | 1 => seq.push(1),  // only works if seq is mutable
            _ => seq.push(&seq[0] + &seq[1]),
        }
    }
    seq
}
```

* Mutable references to data in a vector can be made iff the variable that owns the vector is declared as mutable

```rust
fn fibonacci(n: usize) -> Vec<i32> {
    let mut fib: Vec<i32> = vec![1; n];

    for i in 2..n {
        let new = &fib[i-1] + &fib[i-2];
        // can only declare mutable borrow if the source if mutable
        let cur: &mut i32 = &mut fib[i];
        *cur = new;
    }

    fib
}
```

* Values in a vector can be assigned to another variable if and only if the value itself implemenets the "Copy" trait

```rust
let companies = vec![
    "Facebook".to_string(),
    "Apple".to_string(),
    "Amazon".to_string(),
    "Netflix".to_string(),
    "Google".to_string(),
];

// let zucks_company: String = companies[0] // will not compile
let zucks_company: &mut String = &mut companies[0];
zucks_company.clear(); zucks_company.push_str("Meta");
```

* Values can be assigned into the vector, but the ownership will be moved

```rust
let mut companies = vec![
    "Facebook".to_string(),
    "Apple".to_string(),
    "Amazon".to_string(),
    "Netflix".to_string(),
    "Google".to_string(),
];

let zucks_company: String = "Meta".to_string();
companies[0] = zucks_company;  // the ownership moved

for company in &companies {
    println!("{}", company);
}
```

* Typing in slices

```rust
let nums: Vec<i32> = vec![1, 2, 3, 4, 5];
let slice: &[i32] = &nums;
let elem: i32 = slice[0];  // this is a copy of the value
let elem: &i32 = &slice[0]: // this is an immutable reference
```