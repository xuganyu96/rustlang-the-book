# Notes on structs

## Partial moves
Consider the following snippet

```rust
struct User {
    user_id: u64,
    email: String,
    is_active: bool,
}

fn print_user(user: &User) {
    println!(
        "<User user_id={} email={} active={}>",
        user.user_id, user.email, user.active
    );

}

fn main() {
    let from_: User = User {
        user_id: 0,
        email: String::from("example@email.com"),
        is_active: true,
    };
    print_user(&from_);  // this works

    // Because User.email is heap-allocated, using struct update syntax here
    // causes a memory move; from_.email no longer owns the string values.
    // This is called a "partial move"
    let to: User = User { ..from_ };

    print_user(&to);
    print_user(&from_);  // will not compile
}
```

## Unpacking
Unpacking tuple structs can be done by adding the type name to the expression

```rust
struct RGBColor(i32, i32, i32);

fn print_rgb_color(color: &RGBColor) {
    let RGBColor(r, g, b) = color;
    println!(
        "<RGBColor R={} G={} B={}>", r, g, b
    );
}
```