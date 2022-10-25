use std::env;
use std::error::Error;
use std::fs;

pub struct SearchConfig {
    pub pattern: String,
    pub file_path: String,
    pub case_sensitive: bool,
}

impl SearchConfig {
    /** Return an instance of the SearchConfig struct whose pattern and
     * file_path were sourced from the sequence of arguments passed into its
     * constructor method
     */
    pub fn from_args(args: &[String]) -> Result<Self, &'static str> {
        if args.len() < 3 {
            return Err("Insufficient number of arguments");
        }
        let pattern = args[1].clone();
        let file_path = args[2].clone();
        let case_sensitive = env::var("CASE_SENSITIVE").is_ok();

        Ok(Self { pattern, file_path, case_sensitive })
    }
}

/**
 * Given the search config, perform the search and print the results
 * The type Result<(), Box<dyn Error>> means that this function returns a
 * "Result" enum. The "Ok" variant will return the "unit" type "()", which
 * has exactly one value "()" that indicates "Nothing" (but that is not a
 * NULL!). The "Err" variant will return some type that implements the "Error"
 * trait, but the concrete type will not be known until runtime
 */
pub fn greplite(config: SearchConfig) -> Result<(), Box<dyn Error>> {
    println!("Search config: pattern={} file_path={}", config.pattern, config.file_path);
    let content = fs::read_to_string(config.file_path)?;

    let results = search(&config.pattern, &content, config.case_sensitive);

    for result in results {
        println!("{result}");
    }

    return Ok(());
}

/**
 * Return a vector of string slices where each is a line that contains the
 * input pattern. Each string slice is valid for as long as the content str
 * is valid
 */
pub fn search<'a>(pattern: &str, content: &'a str, case_sensitive: bool) -> Vec<&'a str> {
    let mut results: Vec<&str> = Vec::new();
    let mut pattern = pattern.to_string();
    if !case_sensitive { pattern = pattern.to_lowercase() };

    for line in content.lines() {
        if line.contains(&pattern)
            || (!case_sensitive && line.to_lowercase().contains(&pattern))
        {
            results.push(line);
        }
    }

    return results;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn one_result() {
        let pattern = "duct";
        let content = "\
Rust:
safe, fast, productive.
Pick three.
Duct Tape";

        assert_eq!(vec!["safe, fast, productive."], search(pattern, content, true));
    }

    #[test]
    fn case_insensitive_search() {
        let pattern = "rust";
        let content = "\
Rust:
safe, fast, productive.
Pick three.
Trust me.";

        assert_eq!(vec!["Rust:", "Trust me."], search(pattern, content, false));
    }
}

