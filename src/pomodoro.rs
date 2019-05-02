use crate::input_fn;
use crate::display_time;

pub fn launch() {

    welcome_screen();

    loop {
         let input = input_fn::read_user_input();

        if input_fn::is_input_valid(&input) {

            start_session();

        } else {
            println!("Input not valid! Try again.")
        }
    }
}

fn start_session() {

    const STUDY_TIME: u32 = 25;
    const BREAK_TIME: u32 = 5;

    let mut study_sessions: u32 = 0;

    loop {
        study_sessions += 1;

        display_time::countdown_study(STUDY_TIME);

        if study_sessions % 4 == 0 {

            display_time::countdown_break(BREAK_TIME * 4);

        } else {

            display_time::countdown_break(BREAK_TIME);

        }
    }
}

fn welcome_screen() {

    display_time::clear_console_screen();
    println!("To begin the Study session hit 's'");

}