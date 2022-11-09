//! Function traits describe the behavior of the function-like objects such as
//! functions and closures

/// Reimplementing the "Option" enum to illustrate the idea of function traits
pub enum MyOption<T> {
    Some(T),
    None,
}

impl<T> MyOption<T> {
    /// FnOnce describes functions that are expected to be called exactly
    /// once, most likely because it moves ownership.
    pub fn unwrap_or_else<F>(self, f: F) -> T
    where F: FnOnce() -> T {
        match self {
            MyOption::Some(t) => t,
            MyOption::None => f(),
        }
    }
}

/// Here is another example function that uses some function traits:
/// Given a slice of items of type T and some function that maps item of type
/// to some integer scores, return a copy of the element with the highest
/// score. If the slice if empty, return None
///
/// In this case, the input "key" function will be called on all elements and
/// without mutating any of the elements (since we force T to implement Copy),
/// so the trait "Fn" is appropriate
pub fn generic_max<T, F>(elems: &[T], key: F) -> Option<T>
where
    T: Copy,
    F: Fn(T) -> i32 {
    if elems.len() == 0 {
        return None;
    }
    let mut max_elem: T = elems[0];
    let mut max_score = key(max_elem);

    for &elem in elems {
        let score = key(elem);
        if score > max_score {
            max_score = score;
            max_elem = elem;
        }
    }

    return Some(max_elem);
}

#[cfg(test)]
pub mod tests {
    use super::*;

    #[test]
    fn unwrap_or_else() {
        let some = MyOption::Some(2);
        let none: MyOption<i32> = MyOption::None;
        
        assert_eq!(some.unwrap_or_else(|| 3), 2);
        assert_eq!(none.unwrap_or_else(|| 3), 3);
    }

    #[test]
    fn test_generic_max() {
        let nums = vec![1, 2, 3, 4, 5, 6];
        assert_eq!(generic_max(&nums, |x| x).unwrap(), 6);
        assert_eq!(generic_max(&nums, |x| -x).unwrap(), 1);
    }
}
