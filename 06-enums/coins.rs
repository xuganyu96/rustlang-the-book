#[derive(Debug)]
enum USState {
    CA,
    TX,
    FL,
    NY,
    PA,
}

#[derive(Debug)]
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(USState),
}

fn get_value(coin: &Coin) -> i32 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            match state {
                USState::CA => println!("Californian quarter"),
                USState::TX => println!("Texan quarter"),
                USState::FL => println!("Floridian quarter"),
                USState::NY => println!("New Yorker quarter"),
                USState::PA => println!("Pennsylvanian quarter"),
            }
            25
        }
    }
}

fn main() {
    let coins: [Coin; 8] = [
        Coin::Penny,
        Coin::Nickel,
        Coin::Dime,
        Coin::Quarter(USState::CA),
        Coin::Quarter(USState::TX),
        Coin::Quarter(USState::FL),
        Coin::Quarter(USState::NY),
        Coin::Quarter(USState::PA),
    ];
    let mut coinsum: i32 = 0;

    for (_, coin) in coins.iter().enumerate() {
        // iter() returns ref
        coinsum += get_value(coin);
    }

    println!("{}", coinsum);
}
