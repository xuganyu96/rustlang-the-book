/// Basic usage of an iterator
pub fn basic_iterator() {
    let list = vec![1, 2, 3];
    for item in list.iter() {  // by default iterator returns immutable ref
        println!("{}", item);
    }
}

/// An iterator implements the Iterator trait, which requires the declaration
/// of the "Item" type and the implementation of a "next" method. The "next"
/// method might mutate the internal state of the iterator
pub fn iterator_next() {
    let list = vec![1, 2, 3];
    let mut list_iter = list.iter();
    let elem = list_iter.next();
    println!("{:?}", elem); // Some(1)
    let elem = list_iter.next();
    println!("{:?}", elem); // Some(2)
    let elem = list_iter.next();
    println!("{:?}", elem); // Some(3)
    let elem = list_iter.next();
    println!("{:?}", elem); // None
}

/// The standard library provides some default methods on iterators
/// Some default methods calls the .next() method. Because the .next() method
/// will "use up" the iterator, these methods are called "consuming adapters"
pub fn consuming_adapters() {
    let list  = vec![1, 2, 3];
    let list_iter = list.iter();
    // .sum() will take ownership of the iterator; after .sum() is called,
    // the iterator variable is no longer valid
    let list_sum: i32 = list_iter.sum();
    println!("The list {:?} sums to {}", &list, list_sum);
}

/// Some other methods create an additional iterator on top of the input
/// iterator.
pub fn iterator_adapters() {
    let list = vec![1, 2, 3];
    let list_iter = list.iter();
    let list_mapped: Vec<i32> = list_iter.map(|x| x + 1).collect();

    println!("Mapped list: {:?}", list_mapped);
}

#[derive(Debug)]
struct Shoe {
    size: u32,
    style: String,
}

/// Given (the ownership of) a vector of shoes, return a vector of shoes whose
/// size matches the input shoe size
fn shoes_in_size(shoes: Vec<Shoe>, shoe_size: u32) -> Vec<Shoe> {
    shoes.into_iter().filter(|shoe| shoe.size == shoe_size).collect()
}

pub fn iterator_filter() {
    let shoes = vec![
        Shoe { size: 10, style: "sneaker".to_string() },
        Shoe { size: 13, style: "sandals".to_string() },
        Shoe { size: 10, style: "boots".to_string() },
    ];
    
    let my_size_shoes = shoes_in_size(shoes, 10);

    println!("{:?}", my_size_shoes);
}

