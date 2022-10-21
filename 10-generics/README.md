# Generics

## Review
Let's begin by reviewing some common collections

```rust
fn main() {
    let fibs = vec![1, 1, 2, 3, 5, 8];
    for (i, &num) in (&fibs[..4]).iter().enumerate() {
        println!("{}, {}", i, num);
    }
}
```

