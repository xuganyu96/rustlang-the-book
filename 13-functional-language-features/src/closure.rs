/// A closure is an anonymous function that can be passed as argument to other
/// functions. A prominent use place is Option.unwrap_or_else(...)
use std::thread;

#[derive(Debug, PartialEq, Copy, Clone)]
pub enum ShirtColor {
    Red,
    Blue,
}

pub struct Inventory {
    pub shirts: Vec<ShirtColor>,
}

impl Inventory {
    /// If user_preference is provided, then return user_preference.
    /// If user_preference is None, then return the shirt with the most stock
    pub fn giveaway(&self, user_preference: Option<ShirtColor>) -> ShirtColor {
        user_preference.unwrap_or_else(|| self.most_stocked())
    }

    pub fn most_stocked(&self) -> ShirtColor {
        let mut nreds = 0;
        let mut nblues = 0;

        for color in &self.shirts {
            match color {
                ShirtColor::Red => nreds += 1,
                ShirtColor::Blue => nblues += 1,
            }
        }

        if nreds > nblues { return ShirtColor::Red; }
        return ShirtColor::Blue;
    }
}

/* Closure is usually used in a narrow context where argument and return
 * types can be easily inferred. When needed, type annotation can be still
 * be used.
 */
pub fn closure_type_annotation() {
    let notnice = 68;
    fn increment(x: i32) -> i32 { x + 1 }
    println!("{} increments to {}", notnice, increment(notnice));
    let increment_v2 = |x: i32| -> i32 { x + 1 };  // fully annotated
    println!("{} increments to {}", notnice, increment_v2(notnice));
    let increment_v3 = |x| { x + 1 };  // types inferred from usage below
    println!("{} increments to {}", notnice, increment_v3(notnice));
    let increment_v4 = |x| x + 1;
    println!("{} increments to {}", notnice, increment_v4(notnice));

    /* Closure's argument type is fixed, meaning that one cannot call the same
     * closure with different argument type/return type in different context.
     * Either usage alone is fine, but both present at the same time will lead
     * to compiler error
     */
    let identity = |x| x;
    // println!("Identity({}) is {}", notnice, identity(notnice));
    println!("Identity({}) is {}", "hello", identity("hello"));
}


pub fn closure_capture_values() {
    /* When closure captures variables from the environment, how the variable
     * is captured depends on the content of the closure
     */
    let list = vec![1, 2, 3, 4, 5];
    let immutable_closure = || list.len();
    println!("Length of list is {}", immutable_closure());

    let mut list = vec![1, 2, 3, 4, 5];
    /* Because the closure calls functions that takes mutable reference of
     * list, it captures a mutable reference of the variable. The closure
     * itself needs to be declared "mutable", and the rule of "only one
     * mutable reference" applies
     */
    let mut mutable_closure = |x: i32| list.push(x);
    mutable_closure(6);
    println!("Length of list is {}", list.len());

    /* The last pattern of moving data ownership is useful for running closure
     * in a separate thread. You can force the ownership move with the "move"
     * keyword
     */
    let list = vec![1, 2, 3, 4, 5];
    thread::spawn(move || println!("Length of list is {}", list.len()))
        .join()
        .unwrap();
}

enum MyOption<T> {
    Some(T),
    None,
}

impl<T> MyOption<T> {
    pub fn unwrap_or_else<F>(self, f: F) -> T 
    where 
        F: FnOnce() -> T  // where the Fn traits can be used to specify what
                          // functions or closures this argument can take
    {
        // Because the input argument function will only be called once in the
        // function body, it is appropriate that we only require f to
        // implement "FnOnce"
        match self {
            MyOption::Some(x) => x,
            MyOption::None => f(),
        }
    }
}

pub fn closure_traits() {
    /* There are three traits that describe a closure (and or a function)
     * 
     * FnOnce: the closure can be called once. All closures implement this
     *     trait because all closures can be called at least once. If a
     *     closure moves captured value out of its body, then it can be called
     *     exactly once, so it will implement this trait but not the other two
     * FnMut: the closure can be called more than once. It might or might not
     *     mutate the captured values
     * Fn: the closure doesn't move or mutate captured values, or it doesn't
     *     capture any values. It can be called multiple times concurrently
     */
}
