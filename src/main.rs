#[macro_use]
extern crate run_script;

mod input_fn;
mod pomodoro;
mod display_time;
mod kde_popup;

//This is the default output of the console when the program is opened
fn home_screen() {
    println!("\
              Welcome to Tyler's Pomodoro app, written in Rust\n\
              Select one of the following actions to execute\n
              s : Start Pomodoro with default 25 minute study / 5 minute break\n
              q : Quit execution of this program");
}

fn program_start() {

    home_screen();

    loop {
        let input = input_fn::read_user_input();

        if input_fn::is_input_valid(&input) {

            input_fn::execute_user_input(&input);
            print!("\r");

        } else {
            println!("Input not valid! Try again.");
        }
    }
}


fn main() {
    program_start();
}
