// incluse methods.rs
mod methods;
mod tests;

use std::io;
use std::io::Write;
use colored::Colorize;
use methods::*;
 


// Calculator program
fn main() {
    // // print the title of the program in red color
    // println!("{}", "Calculator".red());
    // // generate a random number between 0 and 100
    // let random_number = rand::thread_rng().gen_range(0..100);
    // // print the random number
    // println!("Random number: {}", random_number);

    //  create a while loop
    loop {
        // print a new line
        println!();
        // run the calculator
        calculator();
        // print a new line
        println!();
        // print the message "Do you want to continue? (y/n)" and flush the output
        print!("{}","Do you want to continue? (y/n) ".yellow());
        io::stdout().flush().expect("Failed to flush");
        
        // create a new string
        let mut answer = String::new();
        // read the answer from the user
        io::stdin()
                    .read_line(&mut answer)
                    .expect("Failed to read line");
        // match the answer
        match answer.trim() {
            // if the answer is 'y'
            "y" => {
                // continue the loop
                continue;
            },
            // if the answer is 'n'
            "n" => {
                // break the loop
                break;
            },
            // if the answer is not one of the above
            _ => {
                // print an error message
                println!("Invalid answer");
                // break the loop
                break;
            }
        }
    }

}


