mod powerline;
mod powerline_central;
mod powerline_naked;
mod powerline_split;
mod trains;

use crate::programinput::ProgramInput;

pub fn dotheme(input: &mut ProgramInput, asname: Option<String>) -> String {
    let tname = match asname {
        Some(a) => a,
        None => input.themename.clone(),
    };

    match tname.as_str() {
        "trains" => trains::dotheme(input),
        "powerline" => powerline::dotheme(input),
        "powerline_naked" => powerline_naked::dotheme(input),
        "powerline_central" => powerline_central::dotheme(input),
        "powerline_split" => powerline_split::dotheme(input),
        "showcase" => showcase(input),
        "indev" => trains::dotheme(input),
        _ => trains::dotheme(input),
    }
}

fn showcase(input: &mut ProgramInput) -> String {
    [
        "trains",
        "powerline",
        "powerline_naked",
        "powerline_central",
        "powerline_split",
    ]
    .into_iter()
    .map(|a| a.to_string())
    .map(|a| format!("\nTheme: \"{}\"\n{}\n", &a, dotheme(input, Some(a.clone()))))
    .reduce(|a, b| format!("{}\n{}", a, b))
    .unwrap_or("Error?".to_string())
}
