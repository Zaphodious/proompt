mod trains;
mod powerline;
mod powerline_naked;
mod powerline_central;
mod powerline_split;

use crate::programinput::ProgramInput;

pub fn dotheme(input: &mut ProgramInput) -> String {
    let tname = input.themename.clone();
    match tname.as_str() {
        "trains" => trains::dotheme(input),
        "powerline" => powerline::dotheme(input),
        "powerline_naked" => powerline_naked::dotheme(input),
        "powerline_central" => powerline_central::dotheme(input),
        "powerline_split" => powerline_split::dotheme(input),
        "indev" => trains::dotheme(input),
        _ => trains::dotheme(input),
    }
}
