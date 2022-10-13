use garden::vegetables::Vegetable;  // brings "Vegetable" into scope

pub mod garden;  // include code in src/garden.rs
                 // which includes code in src/garden/vegetables.rs
                 // which includes the enum garden::vegetables::Vegetable

fn main() {
    let asparagus = Vegetable::Asparagus;

    println!("{:?}", asparagus);
}
