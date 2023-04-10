mod programinput;
mod rgb;
mod themes;
mod gitstatus;
use std::io::Write;


fn main() {
    let mut input = programinput::ProgramInput::new();
    //dbg!(&input);
    let mut output = themes::dotheme(&mut input, None);
    if input.solo_mode {
        output = output.replace("\\[", "").replace("\\]", "");
    }
    std::io::stdout().write(output.as_bytes()).unwrap();
}
