use std::io;
use std::process::exit;

use crate::pomodoro;

pub fn is_input_valid(input: &str) -> bool {

    if input == "s" || input == "q" {
        true
    } else {
        false
    }
}

pub fn read_user_input() -> String {

    let mut input = String::new();
    //all this does is read the line from the console and also has an error
    //handlers, a compile warning suggested it
    let _ = match io::stdin().read_line(&mut input) {
        Ok(input) => input,
        Err(e) => {
            panic!("Error: {}", e)
        },
    };
    //The input needs to be trimmed to remove whitespace and newlines
    input.trim().to_string()
}


pub fn execute_user_input(input: &str) {

    if input == "s" {

        pomodoro::launch();

    } else if input == "q" {

        exit(1);

    }
}