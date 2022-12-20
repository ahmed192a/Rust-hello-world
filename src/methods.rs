
/**
 * This file contains the methods used by the calculator
 */
use std::io::Write;
use colored::*;


// get the operation from the user
pub fn get_operation() -> char {
    //  print the avialbe operaitions ('+', '-', '*', '/')
    print!("Available operations ( +, -, *, / ) : ");
    match std::io::stdout().flush() {
        Ok(_) => {},
        Err(_) => {
            println!("{}", "Failed to flush".red());
        }
    }
    // create a new string
    let mut operation = String::new();
    // read the operation from the user
    match std::io::stdin().read_line(&mut operation){
        Ok(_) => {},
        Err(_) => {
            println!("{}", "Failed to read line".red());
        }
    }
    // return the first character of the string
    let operation: char = operation.trim().chars().next().unwrap();
    // check if the operation is valid if not print an error message and read the operation again
    if operation == '+' || operation == '-' || operation== '*' || operation == '/' {
        operation
    } else {
        println!("{}", "Invalid operation please try again".red());
        get_operation()
    }
}


// get the number from the user
pub fn get_number(numind: String) -> f64 {
    // print insert the first number
    print!("Insert the {} number: ", numind);
    match std::io::stdout().flush() {
        Ok(_) => {},
        Err(_) => {
            println!("{}", "Failed to flush".red());
        }
    }
    // create a new string
    let mut number = String::new();
    // read the number from the user
    match std::io::stdin().read_line(&mut number) {
        Ok(_) => {},
        Err(_) => {
            println!("{}", "Failed to read line".red());
        }
    }
    // convert the string to a float
    match number.trim().parse::<f64>() {
        // if the conversion is successful
        Ok(number) => {
            // return the number
            number
        },
        // if the conversion is not successful
        Err(_) => {
            // print an error message
            println!("{}","Invalid number please try again".red());
            // read the number again
            get_number(numind)
        }
    }
}

// calculate the result
pub fn calculate(operation: char, first_number: f64, second_number: f64) -> f64 {
    // match the operation
    match operation {
        // if the operation is '+'
        '+' => {
            // return the sum of the two numbers
            first_number + second_number
        },
        // if the operation is '-'
        '-' => {
            // return the difference of the two numbers
            first_number - second_number
        },
        // if the operation is '*'
        '*' => {
            // return the product of the two numbers
            first_number * second_number
        },
        // if the operation is '/'
        '/' => {
            // return the quotient of the two numbers
            first_number / second_number
        },
        // if the operation is not one of the above
        _ => {
            // print an error message
            println!("{}", "Invalid operation".red());
            // return 0
            0.0
        }
    }
}

// run the calculator
pub fn calculator() {

    // get the operation from the user
    let operation = get_operation();

    // get the first number from the user
    let first_number = get_number(stringify!("first").to_string());

    // get the second number from the user
    let second_number = get_number(stringify!("second").to_string());
    // calculate the result
    let result = calculate(operation, first_number, second_number);
    // create a string with the equation and the result
    let equation = format!("Result {} {} {} = {}", first_number, operation, second_number, result);
    // print the result in green
    println!("{}",equation.green());
    match std::io::stdout().flush(){
        Ok(_) => {},
        Err(_) => {
            println!("{}", "Failed to flush".red());
        }
    }
}
