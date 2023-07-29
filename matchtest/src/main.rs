// My own atempt at understanding how match works without referring
// to docs

fn main() {
    let grade = 29;

    match grade {
        75 => println!("You scored an A"),
        74 => println!("You scored a B"),
        67 => println!("You scored a C"),
        54 => println!("You scored a D"),
        44 => println!("You scored an E"),
        29 => println!("You scored an F"),
        _ => println!("The number is outside range! Enter number btwn 0 & 100"),
    }
}
