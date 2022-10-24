pub fn add(left: usize, right: usize) -> usize {
    left + right
}

pub fn div(num: usize, den: usize) -> usize {
    if den == 0 { panic!("division by zero is undefined"); }
    return num / den;
}

pub struct Rectangle {
    height: u32,
    width: u32,
}

impl Rectangle {
    /**
     * Recall that child modules can access private methods of the parent
     * module. Even though "can_contain" is not declared public, the test
     * module can test this method
     */
    fn can_contain(&self, other: &Rectangle) -> bool {
        return self.width >= other.width && self.height >= other.height;
    }
}
    

/** The cfg attribute tells Rust to only include the following source code in
 * specific configurations, such as with "cargo test"
 */
#[cfg(test)]
mod tests {
    /* It is necessary to bring definitions from the outer module into the
     * test module
     */
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        /* assert_eq! and assert_ne! are two opposite assertion macros */
        assert_eq!(result, 4);
        assert_ne!(add(2, 2), 5);
    }

    #[test]
    fn larger_contains_smaller() {
        let larger = Rectangle{ height: 10, width: 10 };
        let smaller = Rectangle{ height: 5, width: 5 };
        assert!(larger.can_contain(&smaller));
    }
    
    #[test]
    fn smaller_cannot_contain_larger() {
        let larger = Rectangle{ height: 10, width: 10 };
        let smaller = Rectangle{ height: 5, width: 5 };
        assert!(!smaller.can_contain(&larger));
    }

    #[test]
    #[should_panic(expected="division by zero is undefined")]
    fn zero_division() {
        /* the "should_panic" attribute expects the test to produce a panic,
         * optionally with some expected error message. If the test did not
         * panic, or the panic did not match the expected error message,
         * the test fails
         */
        div(2, 0);
    }

    #[test]
    fn test_with_result() -> Result<(), String> {
        /* Another way to write test is by returning a Result enum. The test
         * suceeds iff the returned value is "Ok". This syntax is very handy
         * when using the "?" operator
         *
         * Do not use #[should_panic] with this kind of tests.
         */
        if 2 + 2 == 4 {
            return Ok(());
        } else {
            return Err(String::from("I have failed elementary arithmetic"));
        }
    }
}
