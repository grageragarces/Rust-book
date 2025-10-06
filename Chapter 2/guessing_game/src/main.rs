// // SIMPLE GUESSER
// use std::io; // input/output library 

// fn main() {
//     println!("Guess the number!");

//     println!("Please input your guess.");

//     let mut guess = String::new(); // mutable variable

//     io::stdin()
//         .read_line(&mut guess)
//         .expect("Failed to read line"); // forced crash

//     println!("You guessed: {guess}");
// }

// // RND N AND SIMPLE GUESSER
// use std::io;

// use rand::Rng;

// fn main() {
//     println!("Guess the number!");

//     let secret_number = rand::thread_rng().gen_range(1..=100);

//     println!("The secret number is: {secret_number}");

//     println!("Please input your guess.");

//     let mut guess = String::new();

//     io::stdin()
//         .read_line(&mut guess)
//         .expect("Failed to read line");

//     println!("You guessed: {guess}");
// }

// // COMPARING GUESS TO RND N

use std::cmp::Ordering;
use std::io;

use rand::Rng; // ordering: less, greater than, equal

fn main() {

    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..=100);

    loop{

        println!("Please input your guess.");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        // let guess: u32 = guess.trim().parse().expect("Please type a number!"); // this method crashes if non number input
        
        // instead in this method you ignore non numb input: switch expect to match
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You guessed: {guess}");

        // trim method on a String eliminates whitespace at the beginning and end: needed as 5 + enter = 5\n
        // u32 = 32 bit integer

        match guess.cmp(&secret_number) { //cmp compares two cals
            // the tree lines bellow are arms (cases of the match)
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break; //close loop
            }
        }
    }
}




