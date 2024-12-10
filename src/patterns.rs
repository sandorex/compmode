use anyhow::{Result, anyhow};

pub type PatternList = &'static [ &'static str ];

pub mod cargo;

pub const ALL: &[&PatternList] = &[
    &cargo::CARGO,
];

pub const GROUPS: &[&str] = &[
    "cargo",
];

#[cfg(test)]
#[test]
fn ensure_groups_are_correct() {
    // NOTE: simple test to ensure i keep groups and all in sync
    assert_eq!(ALL.len(), GROUPS.len());
}

fn pick_by_executable(_exe: &str, _groups: &mut Vec<&PatternList>) {
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

        "make" => pick_by_executable("make", &mut groups),

        // try to find regex group by name
        x => {
            for (index, name) in GROUPS.iter().enumerate() {
                if *name == x.to_lowercase() {
                    groups.push(ALL[index]);
                }
            }

            if groups.is_empty() {
                return Err(anyhow!("Could not find regex group {:?}", x))
            }
        },
    }

    Ok(groups)
}

