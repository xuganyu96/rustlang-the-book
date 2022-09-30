const NICE_NUM: i32 = 69;

fn main() {
    let mut var = 5;
    println!("The value of var is {var}");
    var = 6;
    println!("The value of var is {var}");
    println!("A nice number is {NICE_NUM}");

    let var = "Hello";
    println!("The value of var is {var}");
    let var = var.len();
    println!("The value of var is {var}");

    {
        // Braces opening a new scope is very interesting!
        let var = "world";
        println!("Inside a new scope, the value of var is assigned to {var}");
    }
    println!("Outside the scope, the value of var is {var}");

}
