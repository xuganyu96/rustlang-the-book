//! A T-shirt company periodically gives away free shirts to random users on
//! the mailing list. If the user has indicated a preferred color, then the
//! company will give away that color, else the company will give away the
//! color with the most inventory

#[derive(Debug, PartialEq)]
enum ShirtColor {
    Red,
    Blue,
}

struct Inventory {
    shirts: Vec<ShirtColor>,
}

impl Inventory {
    /// Returns the color with the highest number of occurrences
    fn most_stocked(&self) -> ShirtColor {
        let mut nred = 0;
        let mut nblue = 0;
        for color in &self.shirts {
            match color {
                ShirtColor::Red => nred += 1,
                ShirtColor::Blue => nblue += 1,
            }
        }
        if nred > nblue {
            return ShirtColor::Red;
        }
        return ShirtColor::Blue;
    }

    /// Returns the color of the give-away T-shirt
    fn give_away(&self, user_preference: Option<ShirtColor>) -> ShirtColor {
        user_preference.unwrap_or_else(|| self.most_stocked())
    }
}

#[cfg(test)]
pub mod tests {
    use super::*;

    #[test]
    fn give_away() {
        let inventory = Inventory{
            shirts: vec![
                ShirtColor::Red,
                ShirtColor::Red,
                ShirtColor::Blue,
            ]
        };

        assert_eq!(inventory.give_away(Some(ShirtColor::Red)), ShirtColor::Red);
        assert_eq!(inventory.give_away(None), inventory.most_stocked());
    }
}
