/**
 * A sample program for calculating the area of a rectangle
 */

#[derive(Debug)] // this and {:?} in println! allows printing without
                 // implementing "Display"
struct Rectangle {
    width: u32,
    height: u32,
}

fn get_area(rec: &Rectangle) -> u32 {
    return rec.width * rec.height;
}

fn main() {
    let rec: Rectangle = Rectangle { width: 30, height: 50 };

    println!("The area of rectangle {:#?} is {}", &rec, get_area(&rec));
    dbg!(&rec);  // the debug macro will take ownership (and return)
    let rec: Rectangle = dbg!(rec);
    println!("The area of rectangle {:#?} is {}", &rec, get_area(&rec));
}
