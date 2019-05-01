use std::io;
use crate::pomodoro;
use std::process::exit;

pub fn is_input_valid(input: &str) -> bool {

    if input == "s" || input == "q" {
        true
    } else {
        false
    }
}

pub fn read_user_input() -> String {

    let mut input = String::new();

    io::stdin().read_line(&mut input);
    //The input needs to be trimmed to remove whitespace and newlines
    input.trim().to_string()
}


pub fn execute_user_input(input: &str) {

    if input == "s" {

        pomodoro::launch();

    } else if input == "q" {

        exit(1);

    } else {

        println!("How did you make it here?")
    }
}