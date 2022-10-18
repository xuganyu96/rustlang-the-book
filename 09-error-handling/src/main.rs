mod crashnburn;  // content in crashnburn.rs
mod results;

fn main() {
    // crashnburn::crash_and_burn();
    // crashnburn::bad_index();

    let _file = results::open_or_create("src/main.rs");
    let _file = results::open_or_create("hello.txt");
    // let _file = results::panic_on_error("world.txt");
    println!("{}", results::cat("src/main.rs").expect("Crash and burn!"));
    println!("{}", results::cat_clean("src/main.rs").expect("Crash and burn!"));
}
