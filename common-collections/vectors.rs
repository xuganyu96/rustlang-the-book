enum SpreadsheetCell {
    Integer(i32),
    Float(f64),
    Text(String),
}

use SpreadsheetCell as Cell;

fn println_vec(vec: &Vec<i32>) {
    for i in 0..(vec.len()) {
        match vec.get(i) {
            Some(val) => println!("Element at index {} is {}", i, val),
            None => println!("No element found at index {}", i),
        }
    }
}

fn main() {
    let nums: Vec<i32> = vec![0, 1, 2]; // i32 is the default inferred type
    println_vec(&nums);

    /* when value type cannot be interred, it needs to be explicitly declared
     */
    let mut nums: Vec<i32> = Vec::new();
    nums.push(0); nums.push(1); nums.push(2); // push() requres mutable ref
    println_vec(&nums);


    /* When using "for" loop to iterate over a vector, remember to iterate
     * over (potentially mutable) references instead of the data itself
     */
    for num in &mut nums {
        *num += 1;  // dereference works in the same way in Rust as in C
    }
    println_vec(&nums);


    let cells: Vec<SpreadsheetCell> = vec![
        Cell::Integer(1),
        Cell::Float(3.14),
        Cell::Text(String::from("Hello, world")),
    ];

    for cell in &cells {
        match cell {
            Cell::Integer(val) => println!("Found integer {}", val),
            Cell::Float(val) => println!("Found float {}", val),
            Cell::Text(val) => println!("Found text {}", val),
        }
    }
}

