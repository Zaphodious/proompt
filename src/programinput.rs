use crate::gitstatus::{parse_git_status, GitStatus, StagingStatus};
use crate::rgb::RGB;
use std::env::Args;
use std::fmt::Display;
use std::{env, iter::Peekable};

#[derive(Debug, PartialEq)]
pub struct Section {
    pub foreground: RGB,
    pub background: RGB,
    pub text: String,
    pub starting: String,
    pub ending: String,
    pub specialkind: Option<SpecialKind>,
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
pub enum SpecialKind {
    Git(StagingStatus),
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
    pub gitstatus: Option<GitStatus>,
}

const DEFAULT_STRING: &str = "default";
const DEFAULT_BG: &str = "000000";
const DEFAULT_FG: &str = "111111";

impl ProgramInput {
    pub fn new() -> ProgramInput {
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
            gitstatus: None,
        };
        let mut args = env::args().peekable();

        let mut gitstatus_cache: Option<GitStatus> = None;
        let mut gitdefault_line: String = String::new();

        while let Some(word) = args.next() {
            match word.as_str() {
                "-o" => {
                    input.solo_mode = true;
                }
                "-t" => {
                    let themename = args.next().unwrap_or(DEFAULT_STRING.to_string());
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
                "-s" => input.sections.push(make_section(&mut args, "", None)),
                "--git-s" => {
                    if let Some(g) = &gitstatus_cache {
                        let statuskind = args.next().unwrap();
                        let status = match statuskind.as_str() {
                            "all" => Some(g.staging_status),
                            "committed" => Some(StagingStatus::Committed),
                            "staged" => Some(StagingStatus::AllStaged),
                            "unstaged" => Some(StagingStatus::UnstagedChanges),
                            _ => None,
                        };
                        if let Some(s) = status {
                            if s == g.staging_status {
                                let mut sec = make_section(
                                    &mut args,
                                    gitdefault_line.as_str(),
                                    Some(SpecialKind::Git(s)),
                                );
                                if sec.text != gitdefault_line {
                                    sec.text = g.format_template(sec.text.as_str())
                                }
                                input.sections.push(sec);
                            }
                        }
                    }
                }
                "-i" => {
                    let euid: usize = args.next().unwrap_or("1".to_string()).parse().unwrap_or(1);
                    input.isroot = euid == 0;
                }
                "-c" => {
                    let carrot = args.next().unwrap_or(input.carrot);
                    input.carrot = carrot;
                    let nope = "-nope".to_string();
                    let a = args.peek().unwrap_or(&nope);
                    if !a.starts_with('-') {
                        input.carrotfg = a.as_str().into();
                        args.next();
                    }
                    let a = args.peek().unwrap_or(&nope);
                    if !a.starts_with('-') {
                        input.carrotbg = Some(a.as_str().into());
                        args.next();
                    }
                }
                "-g" => {
                    let status = args.next().unwrap_or("".to_string());
                    let gitstatus = parse_git_status(status);
                    if let Some(g) = &gitstatus {
                        gitdefault_line = g.format_template(" @b ↑@+ ↓@-");
                    }
                    gitstatus_cache = gitstatus.clone();
                    input.gitstatus = gitstatus;
                    //dbg!(status);
                }
                _ => (),
            }
        }

        if let Some(g) = &input.gitstatus {}

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

fn make_section(
    args: &mut Peekable<Args>,
    default_text: &str,
    specialkind: Option<SpecialKind>,
) -> Section {
    let background: RGB = args
        .next()
        .unwrap_or(DEFAULT_BG.to_string())
        .as_str()
        .into();
    let foreground: RGB = args
        .next()
        .unwrap_or(DEFAULT_FG.to_string())
        .as_str()
        .into();
    let text = {
        match args.peek() {
            Some(s) => {
                if s.starts_with("-") {
                    default_text.to_string()
                } else {
                    args.next().unwrap()
                }
            }
            None => default_text.to_string(),
        }
    };
    Section {
        foreground,
        background,
        text,
        starting: "".to_string(),
        ending: "".to_string(),
        specialkind,
    }
}
