use crate::rgb::RGB;
use std::env;
use std::fmt::Display;

#[derive(Debug, PartialEq)]
pub struct Section {
    pub foreground: RGB,
    pub background: RGB,
    pub text: String,
    pub starting: String,
    pub ending: String,
}

impl Display for Section {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "\x1b[48;2;{}m\x1b[38;2;{}m{}{}{}\x1b[0m",
            self.background.to_colcode_frag(),
            self.foreground.to_colcode_frag(),
            self.starting,
            self.text,
            self.ending,
        )
        //println!("\x1b[48;2;{}mHello, world!\x1b[0m\n", rgb.to_colcode_frag());
    }
}

impl Section {
    pub fn len(&self) -> usize {
        self.text.len()
    }
}

#[derive(Debug, PartialEq)]
pub struct ProgramInput {
    pub themename: String,
    pub sections: Vec<Section>,
}

impl ProgramInput {
    pub fn new() -> ProgramInput {
        let defaultstring = String::from("default");
        let defaultBG = String::from("000000");
        let defaultFG = String::from("111111");
        let emptystring = String::new();

        let mut input = ProgramInput {
            themename: defaultstring.clone(),
            sections: Vec::new(),
        };
        let mut args = env::args();

        while let Some(word) = args.next() {
            match word.as_str() {
                "-t" => {
                    let themename = args.next().unwrap_or(defaultstring.clone());
                    input.themename = themename;
                }
                "-s" => {
                    let background: RGB = args.next().unwrap_or(defaultBG.clone()).as_str().into();
                    let foreground: RGB = args.next().unwrap_or(defaultFG.clone()).as_str().into();
                    let text = args.next().unwrap_or(defaultstring.clone());
                    input.sections.push(Section {
                        foreground,
                        background,
                        text,
                        starting: "".to_string(),
                        ending: "".to_string(),
                    });
                }
                _ => (),
            }
        }

        dbg!(&input);
        input
    }
}
