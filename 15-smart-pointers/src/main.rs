use smart_pointers::rcpointers::List;
use std::rc::Rc;

fn main() {
    let list = Rc::new(List::Nil);
    let list = Rc::new(List::Cons(2, list));
    let list = Rc::new(List::Cons(1, list));
    let list = Rc::new(List::Cons(0, list));

    println!("{}", list.straverse());
}
