// My tiny math library
//
#[allow(unused)]
fn get_name(name: String) -> String {
    String::from(name)
}

//special: coz its defn is in another file inside another folder
pub mod financial;

#[allow(unused)]
pub mod math {
    pub mod basic_math {
        fn add_two_numbers(num1: usize, num2: usize) -> usize {
            num1 + num2
        }
        pub fn get_remainder(num1: usize, num2: usize) -> f32 {
            let remainder = num1 % num2;
            remainder as f32
        }
    }

    pub mod area_math {
        pub fn triangle(base: f32, height: f32) -> f32 {
            let area = 0.5 * base * height;
            return area;
        }

        fn circle(radius: f32) -> f32 {
            let area = 3.142 * radius * radius;
            area
        }

        fn rectangle(length: usize, width: usize) -> usize {
            let area = length * width;
            area
        }
    }
}

pub fn area_of_triangle(b: f32, h: f32) -> f32 {
    let area: f32 = math::area_math::triangle(b, h);
    return area;
}

pub fn modulo(fnum: usize, lnum: usize) -> f32 {
    let rem: f32 = crate::math::basic_math::get_remainder(fnum, lnum);
    rem
}

//supe is only used inside a module: debuging 101
#[allow(unused)]
pub mod pname {
    pub fn print_name(username: String) {
        let name = super::get_name(username.to_string());
        println!("{name}");
    }
}

pub use crate as mathlib;
pub use crate::math::area_math;
pub use crate::pname as pnames;
