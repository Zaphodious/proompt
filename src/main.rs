mod programinput;
mod rgb;
use crate::programinput::{ProgramInput, Section};
use crate::rgb::RGB;

fn make_top(sections: &Vec<Section>, space_front: usize, space_end: usize) -> String {
    let lengths: Vec<usize> = sections
        .into_iter()
        .map(|s| -> usize { s.text.len() })
        .collect();
    let mut buf = String::from("\x1b[38;2;80;80;80m╭");
    for len in lengths {
        buf.push('━');
        buf.push_str(str::repeat("═", len-2+space_end+space_front).as_str());
        buf.push('━');
        buf.push('┯');
    }
    buf.pop();
    buf.push('╮');
    return buf;
}

fn make_bottom(sections: &Vec<Section>, space_front: usize, space_end: usize) -> String {
    let lengths: Vec<usize> = sections
        .into_iter()
        .map(|s| -> usize { s.text.len() })
        .collect();
    let mut buf = String::from("\x1b[38;2;80;80;80m╰");
    for len in lengths {
        buf.push_str(str::repeat("┈", len+space_end+space_front).as_str());
        buf.push('╌');
    }
    buf.pop();
    buf.push('╸');
    return buf;
}

fn make_mid(sections: &Vec<Section>, start: &str, mid: &str, end: &str) -> String {
    let starter = "\x1b[38;2;80;80;80m";
    let mut buf = format!("{}{}", starter, start);
    for sec in sections {
        buf.push_str(format!("\x1b[38;2;{}m", sec.background.to_colcode_frag()).as_str());
        buf.push('');
        buf.push_str(sec.to_string().as_str());
        buf.push_str(format!("\x1b[38;2;{}m", sec.background.to_colcode_frag()).as_str());
        buf.push('');
        buf.push_str(starter);
        buf.push_str(mid);
    }
    return buf;
}


fn main() {
    let rgb: RGB = "#600080".into();
    let mut input = ProgramInput::new();
    let top = make_top(&input.sections, 1, 1);
    let bottom = make_bottom(&input.sections, 1, 1);
    let mid = make_mid(&input.sections, "│", "╵", "╯");
    println!("{}", top);
    //    println!("{}", rgb.to_colcode_frag());
    println!("{}", mid);
    println!("{}", bottom);
    //https://www.cyberciti.biz/faq/turn-off-color-in-linux-terminal-bash-session/
    //println!("\x1b[48;2;{}mHello, world!\x1b[0m\n", rgb.to_colcode_frag());
    //    println!("{}", rgb.to_colcode_frag());
}
