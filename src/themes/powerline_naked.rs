use crate::programinput::ProgramInput;
use crate::rgb::CLEARCOL;

fn make_line(input: &ProgramInput) -> String {
    let mut sections = (&input.sections).into_iter().filter(|a|->bool{a.visible});
    let mut buf = format!("{} {} {}",
        input.theme_col().as_foreground(),
        input.carrot,
        input.separators.1,
        );

    while let Some(sec) = sections.next() {
        buf.push_str(sec.primary.as_foreground().as_str());
        buf.push(' ');
        buf.push_str(sec.to_string().as_str());
        buf.push(' ');
        buf.push_str(input.separators.1.as_str());
        //buf.push('');
        buf.push_str(CLEARCOL);
    }
    return buf;
}

pub fn dotheme(input: &mut ProgramInput) -> String {
    return make_line(&input);
}

