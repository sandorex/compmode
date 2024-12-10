use crate::parser::MessageParser;
use anyhow::{Result, anyhow};

pub type PatternList = &'static [ &'static str ];

pub mod cargo;

pub const ALL: &[&PatternList] = &[
    &cargo::CARGO,
];

fn pick_by_executable(exe: &str, groups: &mut Vec<&PatternList>) {
    // TODO for example make / cmake should use gcc / clang etc
}

pub fn pick_group<'a>(group: &str, exe: &str) -> Result<Vec<&'a PatternList>> {
    let mut groups: Vec<&PatternList> = vec![];

    match group.to_lowercase().as_str() {
        "all" => groups.extend(ALL),
        "auto" => {
            pick_by_executable(exe, &mut groups);

            if groups.is_empty() {
                // could not determine the group so use all of them
                groups.extend(ALL);
            }
        },

        "cargo" => groups.push(&cargo::CARGO),

        x => {
            return Err(anyhow!("Could not find regex group {:?}", x))
        },
    }

    Ok(groups)
}

