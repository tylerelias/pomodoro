use std::io;
use std::convert::TryInto;
use std::io::Read;
use std::process::exit;

fn start_custom_pomodoro() {
//    TODO: Implelemt this later
}

fn start_default_pmodoro() {

}

fn execute_user_input(input: &str) {
    if input == "s" {
        println!("Default selected");
        start_default_pmodoro();
    } else if input == "c" {
        println!("Custom selected");
        start_custom_pomodoro();
    } else {
        println!("Quit selected");
        exit(1);
    }
}

fn validate_user_input(input: &str) -> bool {

    if input == "s" || input == "c" || input == "q" {
        true
    } else {
        false
    }
}

fn read_user_input() -> String {
    let mut input = String::new();

    io::stdin().read_line(&mut input);

    input.trim().to_string()
}

//This is the default output of the console when the program is opened
fn home_screen() {
    println!("Welcome to Tyler's Pomodoro app, written in Rust\n\
              Select one of the following actions to execute\n
              s : Start Pomodoro with default 25 minute study / 5 minute break\n
              c : Set a custom study & break length for your study\n
              q : Quit execution of this program");
}

fn program_start() {

    home_screen();

    loop {
        let mut input = read_user_input();

        if validate_user_input(&input) {

            execute_user_input(&input);

        } else {
            println!("Input not valid! Try again.");
        }
    }
}


fn main() {
    program_start();
}
