use crate::rgb::RGB;
use std::env;
use std::fmt::Display;
use std::process::Command;

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
    pub carrot: String,
    pub carrotfg: RGB,
    pub carrotbg: Option<RGB>,
    pub sections: Vec<Section>,
    pub isroot: bool,
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
            carrot: "🮲 🮳".to_string(),
            //carrotfg: "#e00080".into(),
            carrotfg: "#FFFFFF".into(),
            carrotbg: None,
            isroot: false,
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
                    let text = match text.as_str() {
                        "@u" =>  {
                            //let user_id:usize = env::var("EUID").unwrap_or("1").parse().unwrap_or(1);
                            let username = env::var("USER").unwrap_or("batman".to_string());
                            username
                        }
                        _ => text,
                    };
                    input.sections.push(Section {
                        foreground,
                        background,
                        text,
                        starting: "".to_string(),
                        ending: "".to_string(),
                    });
                },
                "-i" => {
                    let euid: usize = args.next().unwrap_or("1".to_string()).parse().unwrap_or(1);
                    input.isroot = euid == 0;
                },
                "-c" => {
                    let carrot = args.next().unwrap_or(input.carrot);
                    input.carrot = carrot;
                }
                "-g" => {
                    
                },
                _ => (),
            }
        }

        input
    }
}
