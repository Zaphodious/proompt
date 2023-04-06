mod programinput;
mod rgb;
use crate::programinput::ProgramInput;
use crate::rgb::RGB;
use rand::prelude::*;
use rand::seq::SliceRandom;
use std::io::Write;

const NEUTRAL_COLOR: &str = "\\[\x1b[38;2;100;100;100m\\]";
const NEUTRAL_BAK: &str = "\\[\x1b[48;2;100;100;100m\\]";
const ROOT_COL: &str = "\\[\x1b[38;2;100;0;0m\\]";
const ROOT_BAK: &str = "\\[\x1b[48;2;100;0;0m\\]";
const CLEARCOL: &str = "\\[\x1b[0m\\]";

const CLOUD_SYMBOLS: &str = "ع˖⁺⋆୭∞*.⋆｡⋆༶⋆˙⊹୭˚○◦˚.˚◦○˚୧";
const TRAINENDS: [&str; 5] = [" ", "█", " █", "█", "█"];

fn rand_clouds(count: usize) -> String {
    let mut rng = thread_rng();
    let syms: Vec<char> = CLOUD_SYMBOLS.chars().choose_multiple(&mut rng, count);
    syms.into_iter().collect()
}

fn rand_train_end() -> &'static str {
    let mut rng = thread_rng();
    TRAINENDS.choose(&mut rng).unwrap_or(&" ")
}

fn determine_neutral_color(input: &ProgramInput) -> &'static str {
    if input.isroot {
        ROOT_COL
    } else {
        NEUTRAL_COLOR
    }
}
fn determine_neutral_bak(input: &ProgramInput) -> &'static str {
    if input.isroot {
        ROOT_BAK
    } else {
        NEUTRAL_BAK
    }
}

fn make_top(input: &ProgramInput, space_front: usize, space_end: usize) -> String {
    let mut buf = format!("{}", determine_neutral_color(input));
    for seg in &input.sections {
        let col = seg.background;
        let darkercol = col
            - RGB {
                r: 70,
                g: 70,
                b: 70,
            };
        let len = seg.text.len();
        buf.push_str(str::repeat(" ", space_front - 3).as_str());
        buf.push_str(darkercol.as_foreground().as_str());
        //buf.push('╭');
        //buf.push_str(str::repeat("═", len - 2).as_str());
        buf.push_str(rand_clouds(len + 1).as_str());
        //buf.push('╮');
        buf.push_str(CLEARCOL);
        buf.push_str(col.as_foreground().as_str());
        buf.push('╖');
        buf.push_str(CLEARCOL);
        buf.push_str(str::repeat(" ", space_end).as_str());
    }
    buf.push_str("\n");
    return buf;
}

fn make_bottom(input: &ProgramInput) -> String {
    let fg = input.carrotfg.as_foreground();
    let bg = if let Some(r) = input.carrotbg {
       r.as_background()
    } else {
        determine_neutral_bak(input).to_string()
    };
    let mut buf = format!(
        "{}█{}{}{} {}{}{}",
        determine_neutral_color(input),
        fg,
        bg,
        input.carrot,
        CLEARCOL,
        determine_neutral_color(input),
        CLEARCOL,
    );
    buf.push_str("\n");

    return buf;
}

fn make_mid(input: &ProgramInput) -> String {
    let sections = &input.sections;
    let mut buf = determine_neutral_color(input).to_string();
    buf.push_str(determine_neutral_bak(input));
    buf.push(' ');
    let mut peaksects = sections.into_iter().peekable();
    while let Some(sec) = peaksects.next() {
        buf.push_str(sec.background.as_foreground().as_str());
        buf.push_str(determine_neutral_bak(input));
        //buf.push('');
        //buf.push('');
        buf.push_str(rand_train_end());
        //buf.push(' ');
        buf.push_str(sec.to_string().as_str());
        buf.push_str(sec.background.as_foreground().as_str());
        if peaksects.peek().is_some() {
            buf.push_str(determine_neutral_bak(input));
        }
        buf.push('');
        buf.push(' ');
        buf.push_str(determine_neutral_color(input));
        if peaksects.peek().is_some() {
            buf.push_str(" ");
        }
        buf.push_str(CLEARCOL);
    }
    buf.push_str(CLEARCOL);
    buf.push_str("\n");
    return buf;
}

fn main() {
    let input = ProgramInput::new();
    let top = make_top(&input, 3, 3);
    let bottom = make_bottom(&input);
    let mid = make_mid(&input);
    let all = [top.as_str(), mid.as_str(), bottom.as_str()].join("");//, "\x1b[0m  "].join("\n");
    std::io::stdout().write(all.as_bytes()).unwrap();
}
