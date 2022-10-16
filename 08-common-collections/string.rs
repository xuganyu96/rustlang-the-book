fn main() {
    /* &str.to_string() and String::from(&str) are interchangeable */
    let strlit: &str = "Hello, world";
    let string: String = strlit.to_string();
    println!("{}", string);
    let string: String = String::from(strlit);
    println!("{}", string);

    /* push_str() and push() both mutate the String */
    let mut string: String = String::new();  // must declare as mutable
    let strlit: &str = "Hello, world";
    string.push_str(strlit);  // push_str takes an immutable reference
    string.push('!');  // push takes immutable reference to a single char
    println!("{}", string);

    /* String::add allows strings to be concatenated, but note its signature:
     * String::add(self, other: &str) -> String
     *
     * Which will deallocate "self" and return (the ownership of) a new String
     * object (so the input reference will be invalid afterwards)
     */
    let mut str1: String = String::from("foo");
    let strlit: &str = "bar";
    str1 = str1 + strlit;
    println!("{}", str1);

    /* &String can be coerced into &str */
    let foo: String = String::from("foo");
    let bar: String = String::from("bar");
    let foobar: String = foo + &bar;  println!("{}", foobar);
    // Another efficient and non-destructive way to concatenate string;
    // since format! takes immutable references, the input references will be
    // valid afterwards
    let foo: String = String::from("foo");
    let bar: String = String::from("bar");
    let foobar: String = format!("{}{}", foo, bar); println!("{}", foobar);
}

