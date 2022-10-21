mod lifetimes;

/* Recall how to implement methods on struct */
struct IntPoint {
    x: i32,
    y: i32,
}

impl IntPoint {
    fn get_coordinates(&self) -> (&i32, &i32) {
        (&self.x, &self.y)
    }
}

/* Generics can be applied to methods, as well */
struct Point<T> {
    x: T,
    y: T,
}

/** By declaring "T" as a generic type after "impl", Rust can identify that
 * the type in the ankle brackets in "Point<T>" is a generic type instead of
 * a concrete type
 */
impl<T> Point<T> {
    fn get_coordinates(&self) -> (&T, &T) {
        (&self.x, &self.y)
    }
}

/* this means that we can ipmlement different methods for concrete types */
// impl Point<i32> {
//     fn get_coordinates(&self) -> (&i32, &i32) {
//         println!("Getting i32 coordiantes!");
//         (&self.x, &self.y)
//     }
// }

mod traits;
// To use the behaviors of a trait, the trait itself needs to be brought into scope
use traits::{NewsArticle, Tweet, Summary};

/** There are several syntax for specifying traits as requirements in functions
 */
fn _notify1(item: &impl Summary) -> String {
    return item.summarize();
}

fn _notify2<T: Summary>(item: &T) -> String {
    return item.summarize();
}

// Specify the multiple traits are required at once
use core::fmt::Display;
fn _multiple_traits1<T: Summary + Display>(item: &T) -> String {
    return format!("{}, {}", item, item.summarize());
}
fn _multiple_traits2(item: &(impl Summary + Display)) -> String {
    return format!("{}, {}", item, item.summarize());
}

// Use "Where" for when trait requirements are complex
fn _complex_traits<T, U>(t: &T, u: &U) -> String
where
    T: Display + Summary,
    U: Summary,
{
    format!("{}, {}, {}", t, t.summarize(), u.summarize())
}

/** You can also specify that a return type implements some traits. Note that
 * using this syntax does not allow the function to return values of different types
 */
fn _return_displayable() -> impl Display {
    return "Hello, world".to_string();

    // you can't do the following
    // if switch { return <some String type> } else { return <some i32 type> }
}

/** Now we can implement the generic "max" function */
fn max<T: PartialOrd>(arr: &[T]) -> Option<&T> {
    if arr.len() == 0 { return None; }
    let mut curmax: &T = &arr[0];
    for elem in arr {
        if elem > curmax {
            curmax = elem;
        }
    }
    return Some(curmax);
}

fn printmax<T: PartialOrd + Display>(arr: &[T]) {
    match max(&arr) {
        None => println!("Array is empty"),
        Some(v) => println!("Max is {}", v),
    }
}

fn main() {
    let origin = IntPoint{ x: 0, y: 0 };
    let (x, y) = origin.get_coordinates();
    println!("<Point x={} y={}>", x, y);

    let point = Point{ x: 1, y: 2 };
    let (x, y) = point.get_coordinates();
    println!("<Point x={} y={}>", x, y);

    let article = NewsArticle {
        author: "author".to_string(),
        headline: "headline".to_string(),
        location: "Earth".to_string(),
        content: "Nothing".to_string(),
    };
    let tweet = Tweet {
        username: "elonmusk".to_string(),
        content: "GameStonk".to_string(),
    };
    println!("{}", article.summarize());
    println!("{}", tweet.summarize());

    let empty: Vec::<i32> = Vec::new();
    let ints = vec![1, 2, 3, 4, 5, 6];
    let floats = vec![1.1, 2.2, 3.3, 4.4, 5.5];
    let chars = vec!['a', 'b', 'c', 'd', 'e'];
    printmax(&empty); printmax(&ints); printmax(&floats); printmax(&chars);
}

