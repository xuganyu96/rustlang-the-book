struct User {
    user_id: u64,
    email: String,
    active: bool,
}

struct RGBColor(i32, i32, i32); // Similar to named tuple
                                // but the fields are unnamed

struct UnitStruct; // a unit-like struct doesn't have any fields

fn print_user(user: &User) {
    println!(
        "<User user_id={} email={} active={}>",
        user.user_id, user.email, user.active
    );
}

fn print_rgb_color(color: &RGBColor) {
    let RGBColor(r, g, b) = color;
    println!(
        "<RGBColor R={} G={} B={}>", r, g, b
    );
}

fn main() {
    let admin: User = User {
        user_id: 0,
        email: String::from("example@email.com"),
        active: true,
    };
    print_user(&admin);

    let lost: User = User {
        user_id: 1,
        email: String::from("example@email.com"),
        active: true,
    };
    let recovered: User = User {
        user_id: 2,
        ..lost
    };
    print_user(&recovered);
    
    let red: RGBColor = RGBColor(255, 0, 0);
    let green: RGBColor = RGBColor(0, 255, 0);
    let blue: RGBColor = RGBColor(0, 0, 255);

    print_rgb_color(&red);
    print_rgb_color(&green);
    print_rgb_color(&blue);

    let unit_struct: UnitStruct = UnitStruct; // instantiating a unit-like
                                              // struct doesn't require parens
}
