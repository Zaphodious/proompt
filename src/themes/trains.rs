use crate::programinput::ProgramInput;
use crate::rgb::{RGB, CLEARCOL};
use rand::prelude::*;
use rand::seq::SliceRandom;

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

fn make_top(input: &ProgramInput, space_front: usize, space_end: usize) -> String {
    let mut buf = input.theme_col_fg().to_string();
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
        input.theme_col_bg()
    };
    let mut buf = format!(
        "{}█{}{}{} {}{}{}",
        input.theme_col_fg(),
        fg,
        bg,
        input.carrot,
        CLEARCOL,
        input.theme_col_fg(),
        CLEARCOL,
    );
    buf.push_str("\n");

    return buf;
}

fn make_mid(input: &ProgramInput) -> String {
    let sections = &input.sections;
    let mut buf = input.theme_col_fg().to_string();
    buf.push_str(input.theme_col_bg().as_str());
    buf.push(' ');
    let mut peaksects = sections.into_iter().peekable();
    while let Some(sec) = peaksects.next() {
        buf.push_str(sec.background.as_foreground().as_str());
        buf.push_str(input.theme_col_bg().as_str());
        //buf.push('');
        //buf.push('');
        buf.push_str(rand_train_end());
        //buf.push(' ');
        buf.push_str(sec.to_string().as_str());
        buf.push_str(sec.background.as_foreground().as_str());
        if peaksects.peek().is_some() {
            buf.push_str(input.theme_col_bg().as_str());
        }
        buf.push('');
        buf.push(' ');
        buf.push_str(input.theme_col_fg().as_str());
        if peaksects.peek().is_some() {
            buf.push_str(" ");
        }
        buf.push_str(CLEARCOL);
    }
    buf.push_str(CLEARCOL);
    buf.push_str("\n");
    return buf;
}

pub fn dotheme(input: &mut ProgramInput) -> String {
    let top = make_top(&input, 3, 3);
    let bottom = make_bottom(&input);
    let mid = make_mid(&input);
    let all = [top.as_str(), mid.as_str(), bottom.as_str()].join("");//, "\x1b[0m  "].join("\n");
    return all; 
}

