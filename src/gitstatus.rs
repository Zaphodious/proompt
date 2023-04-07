use std::{iter::Peekable, str::Split};

#[derive(Default, Debug)]
pub struct GitStatus {
    staging_status: StagingStatus,
    branch: String,
    upstream: String,
    ahead: usize,
    behind: usize,
}

#[derive(Default, Debug, Clone, Copy)]
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
}

pub fn parse_git_status(porcelain: String) {
    let lines: Vec<&str> = porcelain.split("\n").collect();
    let mut status = GitStatus::default();
    dbg!(&lines);
    for line in &lines {
        let words = line.split(" ").peekable();
        parse_status_line(&mut status, words)
    }
    dbg!(&status);
}

fn parse_status_line(status: &mut GitStatus, mut line: Peekable<Split<&str>>) {
    while let Some(word) = line.next() {
        match word {
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
}

//‘ ’	Unmodified
//M	Modified
//A	Added
//D	Deleted
//R	Renamed
//C	Copied
//U	Updated but unmerged
