use oops::pets::{DynamicPetPark, Dog, Cat, Pet};

/// Extend the possibility of pets outside the library's source code!
struct Duck;

impl Pet for Duck {
    fn vocalize(&self) {
        println!("Quack!");
    }
}

fn main() {
    let dog = Dog{};
    let cat = Cat{};
    let duck = Duck{};

    let mut park = DynamicPetPark::new();
    park.admit(Box::new(dog));
    park.admit(Box::new(cat));
    park.admit(Box::new(duck));

    park.greet();
}
