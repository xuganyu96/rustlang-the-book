//! inheritance. Instead, Rust abstracts common behaviors using trait
//! objects

pub trait Pet {
    fn vocalize(&self);
}

pub struct Dog;
pub struct Cat;

impl Pet for Dog {
    fn vocalize(&self) {
        println!("Woof!");
    }
}

impl Pet for Cat {
    fn vocalize(&self) {
        println!("Meow!");
    }
}

/// A park is a collection of pets. Using a trait object allows park.pets to
/// hold items of different types, versus using generics, which means you will
/// have to define a single type at declaration:
///
/// ```
/// pub struct StaticPetPark<T: Pet> {
///    pets: Vec<T>,
/// }
/// ```
pub struct DynamicPetPark {
    // pet park can now hold pets of different types
    pets: Vec<Box<dyn Pet>>,
}

impl DynamicPetPark {
    pub fn new() -> Self {
        return DynamicPetPark{
            pets: vec![],
        };
    }

    pub fn admit(&mut self, pet: Box<dyn Pet>) {
        self.pets.push(pet);
    }

    pub fn greet(&self) {
        for pet in &self.pets {
            pet.vocalize();
        }
    }
}
