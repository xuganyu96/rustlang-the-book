# Questions
## Why does this not work
The idea with the first example is "an element of a singly linked list is either `Nil` or `Cons(i32, <some pointer to another elemnet>)`. The natural thing I come up with is, why couldn't I do this:

```rust
enum Elem {
    Nil,
    Cons(i32, &Elem),
}
```

## Mutable reference behind immutable reference
Consider the example in 15.5: the interface `Messenger` is defined to reflect the requirement that messengers need to be stateless and immutable.

```rust
pub trait Messenger {
    fn emit(&self, msg: &str);
}
```

This creates problem because we want to implement a mock messenger that doesn't conform to this requirement. Instead we want the mock messenger to store emitted messages in a vector for later use.

The book provides an implementation that uses the interior mutability pattern:

```rust
struct MockMessenger {
    messages: RefCell<Vec<String>>,
}
```

But the first thing that came to my mind is this:

```rust
struct MockMessenger<'a> {
    messages: &'a mut Vec<String>,
}

impl<'a> Messenger for MockMessenger<'a> {
    fn emit(&self, msg: &str) {
        self.messages.push(msg.to_string());
    }
}
```

This implementation of `emit` does not compile because `self` is an immutable reference to the messenger, so `self.messages` cannot be borrowed as mutable reference. Here is the rust error message:

```
cannot borrow `*self.messages` as mutable as it is behind a `&` reference
```
