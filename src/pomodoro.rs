use crate::display_time;

pub fn start_session() {

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