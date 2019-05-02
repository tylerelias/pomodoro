use std::io;
use std::{time};
use std::thread::sleep;
use std::io::Write;

use crate::kde_popup;

pub fn countdown_study(duration: u32) {

    clear_console_screen();

    kde_popup::study_start();

    print!("Study Session\n");

    clock(duration);
}

pub fn countdown_break(duration: u32) {

    clear_console_screen();

    print!("Break Session\n");

    if duration == 5 {
        kde_popup::study_break_five();
    } else {
        kde_popup::study_break_twenty();
    }

    clock(duration);
}

fn clock(duration: u32) {

    const ONE_MINUTE: u32 = 60;

    let mut minutes: u32 = duration - 1;
    let mut seconds: u32 = ONE_MINUTE;

    loop {

        sleep(time::Duration::from_secs(1));

        seconds -= 1;

        print_clock(minutes, seconds);

        if seconds == 0 {

            if minutes == 0 { break }

            seconds = ONE_MINUTE;
            minutes -= 1;
        }
    }
}


fn print_clock(minutes: u32, seconds: u32) {

    print!("Time remaining: ");

    if seconds < 10 {
        print!("{}:0{}", minutes, seconds);
    } else {
        print!("{}:{}", minutes, seconds);
    }

    io::stdout().flush().unwrap();
    print!("\r");
}

pub fn clear_console_screen() {
    print!("{}[2J", 27 as char);
    io::stdout().flush().unwrap();
}