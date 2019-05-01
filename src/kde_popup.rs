
pub fn study_start() {
    let (_, _, _) = run_script!(
        r#"
        kdialog --title="Tómaturinn" --icon='tomato' \
        --passivepopup 'Tómaturinn er farin í gang, einbeita sér!'
        "#
    ).unwrap();
}

pub fn study_break_five() {
    let (_, _, _) = run_script!(
        r#"
        kdialog --title="Tómaturinn" --icon='tomato' \
	    --passivepopup 'Tími á 5 mín pásu, njóttu!'
        "#
    ).unwrap();
}

pub fn study_break_twenty() {
    let (_, _, _) = run_script!(
        r#"
        kdialog --title="Tómaturinn" --icon='tomato' \
	    --passivepopup 'Tími á 20 mín pásu, njóttu!'
        "#
    ).unwrap();
}