use crate::programinput::ProgramInput;
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
    let total_secs = &input.sections.len() - 2;
    let center_i: f32 = ((total_secs) / 2) as f32;
    let pre_splits = center_i as usize + 2;
    let post_splits = total_secs - center_i as usize + 2;
    let content_width = (&input.sections)
        .into_iter()
        .filter(|x| x.visible)
        .map(|x| -> usize {
            x.to_string().graphemes(true).count()
        })
        .sum::<usize>()
        + input.separators.0.graphemes(true).count() * post_splits
        + input.separators.1.graphemes(true).count() * pre_splits
        + 1
        ;
    let side_width: usize = input.termwidth.saturating_sub(content_width) / 2;
    buf.push_str(str::repeat(" ", side_width).as_str());
    buf.push_str(&theme_col.as_foreground().as_str());
    buf.push_str(input.separators.0.as_str());
    buf.push_str(&theme_col.as_background());
    buf.push_str(" ");
    let mut last_primary_col = theme_col;
    while let Some((i, sec)) = sections.next() {
        if i as f32 <= center_i {
            buf.push_str(last_primary_col.as_background().as_str());
            buf.push_str(sec.primary.as_foreground().as_str());
            buf.push_str(input.separators.0.as_str());
        }
        buf.push_str(sec.secondary.as_foreground().as_str());
        buf.push_str(sec.primary.as_background().as_str());
        buf.push_str(sec.secondary.as_foreground().as_str());
        buf.push_str(sec.to_string().as_str());
        //buf.push(' ');
        buf.push_str(CLEARCOL);
        if i as f32 >= center_i {
            let bg = if let Some((_, psec)) = sections.peek() {
                psec.primary.as_background()
            } else {
                theme_col.as_background()
            };
            buf.push_str(bg.as_str());
            buf.push_str(sec.primary.as_foreground().as_str());
            buf.push_str(input.separators.1.as_str());
        }
        //buf.push(' ');
        buf.push_str(CLEARCOL);
        last_primary_col = sec.primary;
    }
    buf.push_str(theme_col.as_background().as_str());
    buf.push(' ');
    buf.push_str(CLEARCOL);
    buf.push_str(theme_col.as_foreground().as_str());
    buf.push_str(input.separators.1.as_str());
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
            "{}{}{}{} ",
            bg,
            input.carrot_primary_col.as_foreground().as_str(),
            input.carrot.as_str(),
            CLEARCOL
        )
        .as_str(),
    );
    buf
}

pub fn dotheme(input: &mut ProgramInput) -> String {
    return format!("{}\n{}", make_line(&input), make_carrot(&input));
}
