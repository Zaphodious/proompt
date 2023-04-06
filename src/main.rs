mod programinput;
mod rgb;
mod themes;
use std::io::Write;


fn main() {
    let mut input = programinput::ProgramInput::new();
    let output = themes::dotheme(&mut input);
    std::io::stdout().write(output.as_bytes()).unwrap();
}
