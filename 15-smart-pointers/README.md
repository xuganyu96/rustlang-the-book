# Questions
## Why does this not work
The idea with the first example is "an element of a singly linked list is either `Nil` or `Cons(i32, <some pointer to another elemnet>)`. The natural thing I come up with is, why couldn't I do this:

```rust
enum Elem {
    Nil,
    Cons(i32, &Elem),
}
```

