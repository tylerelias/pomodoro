use crate::input_fn;

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

}

fn welcome_screen() {
    println!("25 min Studying, 5 min breaks.\n\
              Every 4th iteration, you get a 15 min break\n\
              To begin the session hit 's'");

}