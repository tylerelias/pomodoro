use std::io;
use std::{thread, time};
use std::thread::sleep;
use std::cmp::min;

pub fn countdown_study(duration: u64) {

    const ONE_MINUTE: u64 = 60;

    let mut minutes: u64 = duration - 1;
    let mut seconds: u64 = ONE_MINUTE;

    loop {

        sleep(time::Duration::from_secs(1));

        seconds -= 1;

        if seconds == 0 {

            if minutes == 0 { break }

            seconds = ONE_MINUTE;
            minutes -= 1;
        }

        println!("{}:{}", minutes, seconds)
    }

    println!("out of the loop, break call comes here");
    countdown_break(5);
}

pub fn countdown_break(duration: u32) {

}