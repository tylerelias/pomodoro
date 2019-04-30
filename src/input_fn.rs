use std::io;

pub fn validate_user_input(input: &str) -> bool {

    if input == "s" || input == "c" || input == "q" {
        true
    } else {
        false
    }
}

pub fn read_user_input() -> String {
    let mut input = String::new();

    io::stdin().read_line(&mut input);

    input.trim().to_string()
}