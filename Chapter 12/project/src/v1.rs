use std::env;
use std::fs;

fn main() { //performs two tasks: it parses arguments and reads files
    let args: Vec<String> = env::args().collect();
    // Note: std::env::args will panic if any argument contains invalid Unicode -> std::env::args_os (produces OsString values instead of String values)
    let query = &args[1];
    let file_path = &args[2];

    println!("Searching for {query}");
    println!("In file {file_path}");

    let contents = fs::read_to_string(file_path)
        .expect("Should have been able to read the file");

    println!("With text:\n{contents}");
}

// Further iterations to fix four problems:
// 1. scaling -> number of separate tasks the main function handles increases (best to separate functionality -> function = one task)
// 2. longer main -> more vars into scope -> harder keep track of purpose (best to group configuration vars into struct)
// 3. reading file can fail in various ways- missing, no permission to open- same error message for everything!
// 4. user runs program without specifying enough arguments -> error =index out of bounds- that doesn’t clearly explain (best if error-handling code in one place)