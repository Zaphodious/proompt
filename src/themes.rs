mod trains;

use crate::programinput::ProgramInput;

pub fn dotheme(input: &mut ProgramInput) -> String {
    let tname = input.themename.clone();
    match tname.as_str() {
        "trains" => trains::dotheme(input),
        _ => trains::dotheme(input),
    }
}
