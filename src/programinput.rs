use crate::rgb::RGB;
use std::env;
use std::fmt::Display;
use crate::gitstatus::parse_git_status;

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
    pub fn _len(&self) -> usize {
        self.text.len()
    }
}

#[derive(Debug, PartialEq)]
pub struct ProgramInput {
    pub themename: String,
    pub neutral_normal: RGB,
    pub neutral_root: RGB,
    pub motd: String,
    pub carrot: String,
    pub carrotfg: RGB,
    pub carrotbg: Option<RGB>,
    pub sections: Vec<Section>,
    pub isroot: bool,
    pub solo_mode: bool,
}

impl ProgramInput {
    pub fn new() -> ProgramInput {
        let defaultstring = String::from("default");
        let default_bg = String::from("000000");
        let default_fg = String::from("111111");

        let mut input = ProgramInput {
            themename: "trains".to_string(),
            sections: Vec::new(),
            carrot: "🮲 🮳".to_string(),
            //carrotfg: "#e00080".into(),
            carrotfg: "#FFFFFF".into(),
            neutral_normal: "#646464".into(),
            neutral_root: "#640000".into(),
            motd: "Don't forget to be awesome!".to_string(),
            carrotbg: None,
            isroot: false,
            solo_mode: false,
        };
        let mut args = env::args().peekable();

        while let Some(word) = args.next() {
            match word.as_str() {
                "-o" => {
                    input.solo_mode = true;
                }
                "-t" => {
                    let themename = args.next().unwrap_or(defaultstring.clone());
                    input.themename = themename;
                    if let Some(a) = args.peek() {
                        if !a.starts_with('-') {
                            input.neutral_normal = a.as_str().into();
                            args.next();
                        }
                    }
                    if let Some(a) = args.peek() {
                        if !a.starts_with('-') {
                            input.neutral_root = a.as_str().into();
                            args.next();
                        }
                    }
                    if let Some(a) = args.peek() {
                        if !a.starts_with('-') {
                            input.motd = a.clone();
                            args.next();
                        }
                    }
                }
                "-s" => {
                    let background: RGB = args.next().unwrap_or(default_bg.clone()).as_str().into();
                    let foreground: RGB = args.next().unwrap_or(default_fg.clone()).as_str().into();
                    let text = args.next().unwrap_or(defaultstring.clone());
                    input.sections.push(Section {
                        foreground,
                        background,
                        text,
                        starting: "".to_string(),
                        ending: "".to_string(),
                    });
                }
                "-i" => {
                    let euid: usize = args.next().unwrap_or("1".to_string()).parse().unwrap_or(1);
                    input.isroot = euid == 0;
                }
                "-c" => {
                    let carrot = args.next().unwrap_or(input.carrot);
                    input.carrot = carrot;
                },
                "-g" => {
                    let status = args.next().unwrap_or("".to_string());
                    parse_git_status(status)
                    //dbg!(status);
                }
                _ => (),
            }
        }

        input
    }
    pub fn theme_col_fg(&self) -> String {
        if self.isroot {
            self.neutral_root.as_foreground()
        } else {
            self.neutral_normal.as_foreground()
        }
    }
    pub fn theme_col_bg(&self) -> String {
        if self.isroot {
            self.neutral_root.as_background()
        } else {
            self.neutral_normal.as_background()
        }
    }
}
