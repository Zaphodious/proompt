use crate::gitstatus::{parse_git_status, GitStatus, StagingStatus};
use crate::rgb::RGB;
use std::env::Args;
use std::fmt::Display;
use std::{env, iter::Peekable};

#[derive(Debug, PartialEq)]
pub struct Section {
    pub primary: RGB,
    pub secondary: RGB,
    pub text: String,
    pub starting: String,
    pub ending: String,
    pub sectiontype: SectionType,
    pub visible: bool,
}

impl Display for Section {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}{}{}",
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
pub enum SectionType {
    Normal,
    Git(StagingStatus),
    Break
}

#[derive(Debug, PartialEq)]
pub struct ProgramInput {
    pub themename: String,
    pub neutral_normal: RGB,
    pub neutral_root: RGB,
    pub motd: String,
    pub carrot: String,
    pub carrot_primary_col: RGB,
    pub carrot_secondary_col: Option<RGB>,
    pub sections: Vec<Section>,
    pub isroot: bool,
    pub solo_mode: bool,
    pub gitstatus: Option<GitStatus>,
    pub termwidth: usize,
    pub separators: (String, String),
}

const DEFAULT_STRING: &str = "default";
const DEFAULT_BG: &str = "000000";
const DEFAULT_FG: &str = "ffffff";

impl ProgramInput {
    pub fn new() -> ProgramInput {
        let mut input = ProgramInput {
            themename: "trains".to_string(),
            sections: Vec::new(),
            carrot: "🮲 🮳".to_string(),
            //carrotfg: "#e00080".into(),
            carrot_primary_col: DEFAULT_FG.into(),
            neutral_normal: "#646464".into(),
            neutral_root: "#640000".into(),
            motd: "Don't forget to be awesome!".to_string(),
            carrot_secondary_col: None,
            isroot: false,
            solo_mode: false,
            gitstatus: None,
            termwidth: 80,
            separators: ("".to_string(), "".to_string()),
        };
        let mut args = env::args().peekable();

        let mut gitstatus_cache: Option<GitStatus> = None;
        let mut gitdefault_line: String = String::new();

        while let Some(word) = args.next() {
            match word.as_str() {
                "-o" => {
                    input.solo_mode = true;
                },
                "-w" => {
                    if let Some(w_s) = args.next() {
                        if let Ok(w) = w_s.parse::<usize>() {
                            input.termwidth = w;
                        }
                    }
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
                "-s" => input.sections.push(make_section(&mut args, "", SectionType::Normal)),
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
                                    SectionType::Git(s),
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
                        input.carrot_primary_col = a.as_str().into();
                        args.next();
                    }
                    let a = args.peek().unwrap_or(&nope);
                    if !a.starts_with('-') {
                        input.carrot_secondary_col = Some(a.as_str().into());
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
                },
                "--separators" => {
                    let right = args.next().unwrap_or(input.separators.0);
                    let left = args.next().unwrap_or(input.separators.1);
                    input.separators = (right, left);
                },
                "--break" => {
                    let sec: Section = Section {
                        primary: DEFAULT_BG.into(),
                        secondary: DEFAULT_FG.into(),
                        text: "BREAK".to_string(),
                        ending: ">>>".to_string(),
                        starting: "<<<".to_string(),
                        sectiontype: SectionType::Break,
                        visible: false,
                    };
                    input.sections.push(sec);
                }
                _ => (),
            }
        }

        input
    }
    pub fn theme_col(&self) -> RGB {
        if self.isroot {
            self.neutral_root
        } else {
            self.neutral_normal
        }
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
    sectiontype: SectionType,
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
        primary: background,
        secondary: foreground,
        text,
        starting: "".to_string(),
        ending: "".to_string(),
        sectiontype,
        visible: true,
    }
}
