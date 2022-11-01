use functional_features::closure::{self, Inventory, ShirtColor};

fn main() {
    let inventory = Inventory {
        shirts: vec![ShirtColor::Red, ShirtColor::Red, ShirtColor::Blue],
    };

    let user_preference = Some(ShirtColor::Blue);
    println!("{:?}", inventory.giveaway(user_preference));

    let user_preference = None;
    println!("{:?}", inventory.giveaway(user_preference));

    closure::closure_type_annotation();
    closure::closure_capture_values();
}
