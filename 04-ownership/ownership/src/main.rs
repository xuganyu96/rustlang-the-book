/**
 * &str is a string literal. The size of a string literal is determined
 * at compile time just like all other primitive types.
 * String is the mutable string type. The size of a "String" cannot be
 * determined at compile time (e.g. user input). "String" variables are
 * heap-allocated
 */
fn heapstr(strlit: &str) {
    let _heapstr = String::from(strlit);
    println!("{}", _heapstr);
    // when this function exits, Rust will automatically free the heap memory
    // allocated to _heapstr
}

/**
 * A second way Rust ensures memory safety is by forcing exactly one
 * pointer to a heap memory address to exist at within a frame at any time.
 * Having two pointers to the same heap memory address is dangerous because
 * it can lead to freeing freed heap memory
 */
fn heapmove() {
    let str1: String = String::from("Hello, world");
    let str2: String = str1;

    // println!("{}", str1);  // will not compile: str1 is already freed
    println!("{}", str2);
    // This is called a "move" since the ownership of the data is moved from
    // one variable to another

    let strclone = str2.clone();
    println!("{}", strclone);
}

/**
 * When a heap-allocated memory address is passed into a function, its
 * ownership is moved into the function as well. As a result, the variable
 * on the outer frame is invalidated.
 */
fn take_ownership(new: String) {
    println!("{}", new);
    // I think the heap allocated string will actually also be freed when the
    // functino exits
}

/**
 * Conversely, a function can give the ownership of some heap-allocated memory
 * to an outer scope by returning the value (of that heap-allocated memory's
 * address)
 */
fn give_ownership() -> String {
    let oldowner: String = String::from("Hello, world");
    return oldowner;
}

fn main() {
    // &str is the immutable string literal
    let strlit: &str = "Hello, world";
    heapstr(strlit);
    heapmove();

    let oldowner: String = String::from(strlit);
    take_ownership(oldowner);
    // println!("{}", oldowner);  // will not compile

    let newowner: String = give_ownership();
    println!("{newowner}");
}
