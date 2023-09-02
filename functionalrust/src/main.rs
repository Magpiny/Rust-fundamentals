/*
*   FUNCTIONAL PROGRAMMING
* Closures: Anonymous functions that capture their environment
*Rust’s closures are anonymous functions you can save in a variable
or pass as arguments to other functions.
*
* Iterators
*
*/

#[derive(Debug, PartialEq, Copy, Clone)]
enum ShirtColor {
    Green,
    Yellow,
}

struct Inventory {
    shirts: Vec<ShirtColor>,
}

impl Inventory {
    fn giveaway(&self, user_preference: Option<ShirtColor>) -> ShirtColor {
        user_preference.unwrap_or_else(|| self.most_stocked())
    }

    fn most_stocked(&self) -> ShirtColor {
        let mut num_green = 0;
        let mut num_yellow = 0;

        for color in &self.shirts {
            match color {
                ShirtColor::Green => num_green += 1,
                ShirtColor::Yellow => num_yellow += 1,
            }
        }
        if num_green > num_yellow {
            ShirtColor::Green
        } else {
            ShirtColor::Yellow
        }
    }
}

fn main() {
    let store = Inventory {
        shirts: vec![ShirtColor::Yellow, ShirtColor::Green, ShirtColor::Yellow],
    };

    let user_pref1 = Some(ShirtColor::Green);
    let giveaway1 = store.giveaway(user_pref1);
    println!(
        "The user with preference {:?} gets {:?}",
        user_pref1, giveaway1
    );

    let user_pref2 = None;
    let giveaway2 = store.giveaway(user_pref2);
    println!(
        "The user with preference {:?} gets {:?}",
        user_pref2, giveaway2
    );
}
