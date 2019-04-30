use std::io;
use std::process::exit;

mod input_fn;
mod default_pomodoro;
mod display_time;

fn start_custom_pomodoro() {
//    TODO: Implelemt this later
}


fn execute_user_input(input: &str) {

    if input == "s" {

        default_pomodoro::launch();

    } else if input == "c" {

        println!("Custom selected");
        start_custom_pomodoro();

    } else {

        println!("Quit selected");
        exit(1);

    }
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
        let mut input = input_fn::read_user_input();

        if input_fn::validate_user_input(&input) {

            execute_user_input(&input);

        } else {
            println!("Input not valid! Try again.");
        }
    }
}


fn main() {
    program_start();
}
