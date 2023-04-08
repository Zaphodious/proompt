use std::{iter::Peekable, str::Split, fmt::Display};

#[derive(Default, Debug, Eq, PartialEq, PartialOrd, Ord, Clone)]
pub struct GitStatus {
    pub staging_status: StagingStatus,
    pub branch: String,
    pub upstream: String,
    pub ahead: usize,
    pub behind: usize,
}

impl GitStatus {
    pub fn format_template(&self, template_string: &str) -> String {
        template_string
            .replace("@b", self.branch.as_str())
            .replace("@u", self.upstream.as_str())
            .replace("@i", self.staging_status.get_icon().as_str())
            .replace("@e", self.staging_status.get_emoji().as_str())
            .replace("@s", self.staging_status.to_string().as_str())
            .replace("@+", self.ahead.to_string().as_str())
            .replace("@-", self.behind.to_string().as_str())
    }
}

#[derive(Default, Debug, Clone, Copy, Eq, PartialEq, PartialOrd, Ord)]
pub enum StagingStatus {
    UnstagedChanges,
    AllStaged,
    #[default]
    Committed,
}

impl StagingStatus {
    fn check_override(&self, other: Self) -> StagingStatus {
        match self {
            Self::UnstagedChanges => Self::UnstagedChanges,
            Self::AllStaged => match other {
                Self::UnstagedChanges => Self::UnstagedChanges,
                _ => Self::AllStaged,
            },
            Self::Committed => other,
        }
    }

    fn get_icon(&self) -> String {
        match self {
            Self::UnstagedChanges => "🗴",
            Self::AllStaged => "•",
            Self::Committed => "✔",
        }.to_string()
    }
    fn get_emoji(&self) -> String {
        match self {
            Self::UnstagedChanges => "🤨",
            Self::AllStaged => "😅",
            Self::Committed => "😎",
        }.to_string()

    }
}

impl Display for StagingStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::UnstagedChanges => f.write_str("Unstaged"),
            Self::AllStaged => f.write_str("Staged"),
            Self::Committed => f.write_str("Committed"),
        }
    }
}

//" @b ↑@+ ↓@-"

pub fn parse_git_status(porcelain: String) -> Option<GitStatus> {
    let lines: Vec<&str> = porcelain.split("\n").collect();
    let mut status = GitStatus::default();
    //dbg!(&lines);
    for line in &lines {
        let words = line.split(" ").peekable();
        parse_status_line(&mut status, words)?
    }
    //dbg!(&status);
    return Some(status);
}

fn parse_status_line(status: &mut GitStatus, mut line: Peekable<Split<&str>>) -> Option<()> {
    while let Some(word) = line.next() {
        match word {
            "fatal:" => {
                return None;
            }
            "#" => {
                if let Some(headername) = line.next() {
                    match headername {
                        "branch.head" => {
                            status.branch = line.next().unwrap_or("error").to_string();
                        }
                        "branch.upstream" => {
                            status.upstream = line.next().unwrap_or("error").to_string();
                        }
                        "branch.ab" => {
                            let mut aheadstr = line.next().unwrap_or("error").to_string();
                            aheadstr.remove(0);
                            status.ahead = aheadstr.parse().unwrap_or(69420);
                            let mut behindstr = line.next().unwrap_or("error").to_string();
                            behindstr.remove(0);
                            status.behind = behindstr.parse().unwrap_or(69420);
                        }
                        _ => (),
                    }
                }
            }
            "1" => {
                let mut smessage = line.next().unwrap_or("..").chars();
                if smessage.next().unwrap() == '.' {
                    status.staging_status = status
                        .staging_status
                        .check_override(StagingStatus::UnstagedChanges)
                } else if smessage.next().unwrap() == '.' {
                    status.staging_status = status
                        .staging_status
                        .check_override(StagingStatus::AllStaged)
                }
            }
            _ => (),
        }
    }
    Some(())
}

//‘ ’	Unmodified
//M	Modified
//A	Added
//D	Deleted
//R	Renamed
//C	Copied
//U	Updated but unmerged
