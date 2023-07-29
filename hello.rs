// Wed 14th June 2023 0416hrs
// hELLO world program
// 

/* 
Rust is an ahead-of-time compiled language, meaning you can compile a program and give the executable 
to someone else, and they can run it even without having Rust installed. If you give someone a .rb, .py, 
or .js file, they need to have a Ruby, Python, or JavaScript implementation installed (respectively). 
But in those languages, you only need one command to compile and run your program. Everything is a 
trade-off in language design.

*/

/*
*Printing to console:

    format!: write formatted text to String
    print!: same as format! but the text is printed to the console (io::stdout).
    println!: same as print! but a newline is appended.
    eprint!: same as print! but the text is printed to the standard error (io::stderr).
    eprintln!: same as eprint! but a newline is appended.

*
* */

fn main() {
    let mut number = 10;
    while number>0 {
        println!("Count: {number}");
        number -= 1;
    }
}

