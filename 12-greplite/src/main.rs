use std::env;
use std::process;

use greplite::{ self, SearchConfig };

fn main() {
    let args: Vec<String> = env::args().collect();
    let config = SearchConfig::from_args(&args).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {err}");
        process::exit(1);
    });

    /* "if let" is used instead of "unwrap" or "expect" because we expect
     * the function greplite() to return no meaningful result; instead, we
     * only care for when the function fails
     */
    if let Err(e) = greplite::greplite(config) {
        eprintln!("The search failed: {e}");
        process::exit(1);
    }
}

