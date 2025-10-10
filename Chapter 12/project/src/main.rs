use std::env;
use std::fs;
use std::process;
use std::error::Error;
use project::search; //Splitting Code into a Library Crate

fn main() {
    let args: Vec<String> = env::args().collect();

    // handle the Result being returned by Config::build
    let config = Config::build(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {err}");
        process::exit(1);
    });

    println!("Searching for {}", config.query);
    println!("In file {}", config.file_path);

    if let Err(e) = run(config) {
        println!("Application error: {e}");
        process::exit(1);
    }
}

// fn run(config: Config) {
//     let contents = fs::read_to_string(config.file_path)
//         .expect("Should have been able to read the file");

//     println!("With text:\n{contents}");
// }

// // allows the program to panic by calling expect -> run function returns Result<T, E> when something goes wrong
// fn run(config: Config) -> Result<(), Box<dyn Error>> {
//     let contents = fs::read_to_string(config.file_path)?;

//     println!("With text:\n{contents}");

//     Ok(())
// }

//Splitting Code into a Library Crate
fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.file_path)?;

    for line in search(&config.query, &contents) {
        println!("{line}");
    }

    Ok(())
}

struct Config {
    query: String,
    file_path: String,
}

// impl Config {
//     fn new(args: &[String]) -> Config { //FUN FACT: many programmers expect new functions to never fail
//         if args.len() < 3 {
//             panic!("not enough arguments");
//         }

//         let query = args[1].clone();
//         let file_path = args[2].clone();

//         Config { query, file_path }
//     }
// }

// return Config instance in the successful case and will describe the problem in the error case
impl Config {
    fn build(args: &[String]) -> Result<Config, &'static str> { //Result type to signal there was a problem
        if args.len() < 3 {
            return Err("not enough arguments");
        }

        let query = args[1].clone();
        let file_path = args[2].clone();

        Ok(Config { query, file_path })
    }
}