use crate::input_fn;
use crate::display_time;

const BREAK_TIME: u32 = 5;

pub fn launch() {

    welcome_screen();

    loop {
         let mut input = input_fn::read_user_input();

        if input_fn::validate_user_input(&input) {

            start_session();

        } else {
            println!("Input not valid! Try again.")
        }
    }
}

fn start_session() {

    const STUDY_TIME: u64 = 25;

    let study_sessions: u64 = 0;

    display_time::countdown_study(STUDY_TIME)
}

fn welcome_screen() {
    println!("25 min Studying, 5 min breaks.\n\
              Every 4th iteration, you get a 15 min break\n\
              To begin the session hit 's'");

}