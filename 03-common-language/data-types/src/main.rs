fn main() {
    let text = "69";
    let num: i32 = text.parse().expect("Error message");

    println!("text is {text}, which can be parsed to {num}");
}
