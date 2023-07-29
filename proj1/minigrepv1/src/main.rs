//An IO projec:t
//Building a COMMANDLINE PROJECT
//V1: Improved -> Some refactpring done to improve the code
//
//
use std::{env, fs, process};

fn main() {
    let args: Vec<String> = env::args().collect();
    let config = Config::build(&args).unwrap_or_else(|err| {
        println!("Probem passing arguments: {err}");
        process::exit(1);
    });
    println!("Searching for {} in {}", config.query, config.file_path);

    let contents = fs::read_to_string(config.file_path).expect("Unable to read file!");
    println!("With Contents \n {contents}");
}

struct Config {
    query: String,
    file_path: String,
}

// i like this
impl Config {
    fn build(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("Not enough arguments!");
        }
        let query = args[1].clone();
        let file_path = args[2].clone();

        return Ok(Config { query, file_path });
    }
}
