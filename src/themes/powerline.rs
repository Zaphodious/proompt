use crate::programinput::ProgramInput;
use crate::rgb::CLEARCOL;

fn make_line(input: &ProgramInput) -> String {
    let mut sections = (&input.sections).into_iter().filter(|a|->bool{a.visible}).peekable();
    let theme_col = input.theme_col();
    let mut buf = format!("{}{} {} {}{}{} ",
            (&theme_col).as_background(),
            &input.carrot_primary_col.as_foreground(),
            &input.carrot,
            sections.peek().unwrap().primary.as_background(),
            (&theme_col).as_foreground(),
            ''
                          );
    //buf.push_str(input.theme_col_bg().as_str());
    //buf.push(' ');
    while let Some(sec) = sections.next() {
        buf.push_str(sec.secondary.as_foreground().as_str());
        buf.push_str(sec.primary.as_background().as_str());
        buf.push_str(sec.secondary.as_foreground().as_str());
        buf.push_str(sec.to_string().as_str());
        buf.push_str(CLEARCOL);
        if let Some(psec) = sections.peek() {
            buf.push_str(psec.primary.as_background().as_str());
        }         
        buf.push_str(sec.primary.as_foreground().as_str());
        buf.push_str(input.separators.1.as_str());
        buf.push_str(CLEARCOL);
    }
    return buf;
}

pub fn dotheme(input: &mut ProgramInput) -> String {
    return make_line(&input);
}

