/* the "Result<T, E>" prelude is already imported in the prelude */
use std::fs::File;
use std::io::{self, ErrorKind, Read};

/**
 * Open and return a file handle at the specified path, unless the file does
 * not exist, then create the file and return a handle. If anything else goes
 * wrong, the function will panic
 */
pub fn open_or_create(path: &str) -> File {  // not Option because panic
    match File::open(path) {
        Ok(file) => {
            println!("File {:?} opened", file);
            file
        },
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create(path) {
                Ok(newfile) => {
                    println!("File {:?} created", newfile);
                    newfile
                },
                _ => panic!("Could not create new file {}", path),
            },
            _ => panic!("Could not open file {}", path),
        }
    }
}

/** A shortcut for "panic on Result::Err" is unwrap or expect; "expect" is
 * better because it allows more specificity
 */
pub fn panic_on_error(path: &str) -> File {
    let f = File::open(path).expect(&format!("{} failed to open", path));
    return f;
}

/** another pattern for handling error is to return it to the caller
 * "head" reads the first line of the input file, unless the file fails to
 * open, then the error is returned
 */
pub fn cat(path: &str) -> Result<String, io::Error> {
    let mut f = match File::open(path) {
        Ok(file) => file,
        Err(error) => return Err(error),
    };

    let mut line = String::new();

    match f.read_to_string(&mut line) {
        Ok(_) => Ok(line),
        Err(e) => Err(e),
    }
}

/** A short hand for handling Result enum type is the "?" operator
 * The "?" operator will force an early return, so pay extra attention to
 * the return type of the function. When applied to "Result", the "?" operator
 * will return "Result"; when applied to "Option", the "?" operator will
 * return "Option"
 */
pub fn cat_clean(path: &str) -> Result<String, io::Error> {
    let mut content = String::new();

    File::open(path)?.read_to_string(&mut content)?;

    return Ok(content);
}
