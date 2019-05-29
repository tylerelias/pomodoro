#[macro_use]
extern crate run_script;

use std::process::exit;

mod pomodoro;
mod display_time;
mod kde_popup;

fn main() {
    while true {
        pomodoro::start_session();
    }
    exit(1);
}
