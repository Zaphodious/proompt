use crate::programinput::{ProgramInput, SectionType};
use crate::rgb::CLEARCOL;
use unicode_segmentation::UnicodeSegmentation;

fn make_line(input: &ProgramInput) -> String {
    let mut sections = (&input.sections)
        .into_iter()
        .filter(|a| a.visible)
        .enumerate()
        .peekable();
    let theme_col = input.theme_col();
    let mut buf = String::from(CLEARCOL);
    //buf.push_str(input.theme_col_bg().as_str());
    //buf.push(' ');
    let total_secs = &input.sections.len();
    let split_position = if let Some((n, _)) = (&input.sections)
        .into_iter()
        .enumerate()
        .filter(|(_, a)| a.sectiontype == SectionType::Break)
        .next()
    {
        n - 1
    } else {
        (total_secs - 1) / 2
    };
    let pre_splits = split_position + 1;
    let post_splits = total_secs - split_position - 2;
    let pre_sep_count = input.separators.0.graphemes(true).count() * pre_splits;
    let post_sep_count = input.separators.0.graphemes(true).count() * post_splits;
    let content_width = (&input.sections)
        .into_iter()
        .filter(|x| x.visible)
        .map(|x| -> usize {
            x.to_string().graphemes(true).count()
        })
        .sum::<usize>()
        + pre_sep_count
        + post_sep_count
        //+ 1
        ;
    let split_width = input.termwidth.saturating_sub(content_width);
    buf.push_str(&theme_col.as_foreground().as_str());
    //buf.push('');
    //buf.push_str(&theme_col.as_background());
    //buf.push_str(" ");
    let mut last_primary_col = theme_col;
    while let Some((i, sec)) = sections.next() {
        if i > split_position {
            if i == split_position + 1 {
                buf.push_str(CLEARCOL);
            } else {
                buf.push_str(last_primary_col.as_background().as_str());
            }
            buf.push_str(sec.primary.as_foreground().as_str());
            buf.push_str(input.separators.0.as_str())
        }
        buf.push_str(sec.secondary.as_foreground().as_str());
        buf.push_str(sec.primary.as_background().as_str());
        buf.push_str(sec.secondary.as_foreground().as_str());
        buf.push_str(sec.to_string().as_str());
        //buf.push(' ');
        buf.push_str(CLEARCOL);
        if i < split_position {
            let bg = if let Some((_, psec)) = sections.peek() {
                psec.primary.as_background()
            } else {
                theme_col.as_background()
            };
            buf.push_str(bg.as_str());
            buf.push_str(sec.primary.as_foreground().as_str());
            buf.push_str(input.separators.1.as_str());
        }
        if i == split_position {
            buf.push_str(CLEARCOL);
            buf.push_str(sec.primary.as_foreground().as_str());
            buf.push_str(input.separators.1.as_str());
            buf.push_str(CLEARCOL);
            buf.push_str(" ".repeat(split_width).as_str());
        }
        //buf.push(' ');
        buf.push_str(CLEARCOL);
        last_primary_col = sec.primary;
    }
    /*
     *
    buf.push_str(theme_col.as_background().as_str());
    buf.push(' ');
    buf.push_str(CLEARCOL);
    buf.push_str(theme_col.as_foreground().as_str());
    buf.push('');
     */
    buf.push_str(CLEARCOL);
    return buf;
}

fn make_carrot(input: &ProgramInput) -> String {
    let mut buf = String::new();
    let bg = if let Some(c) = input.carrot_secondary_col {
        c.as_background()
    } else {
        "".to_string()
    };
    buf.push_str(
        format!(
            "{}{}{}{} {}{}{}{}",
            input.theme_col().as_background(),
            input.carrot_primary_col.as_foreground().as_str(),
            bg,
            input.carrot.as_str(),
            CLEARCOL,
            input.theme_col().as_foreground(),
            input.separators.1,
            CLEARCOL,
        )
        .as_str(),
    );
    buf
}

pub fn dotheme(input: &mut ProgramInput) -> String {
    return format!("{}\n{}", make_line(&input), make_carrot(&input));
}
