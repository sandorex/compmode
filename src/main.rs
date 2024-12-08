mod cli;
mod groups;

use anyhow::Context;
use clap::Parser;
use regex::Regex;
use std::{io::{BufRead, BufReader}, process::{Command, Stdio}};

pub mod prelude {
    pub use anyhow::{anyhow, Context as AnyhowContext, Result};
}

/// Contains one message with its information, used for both errors and warnings
#[derive(Debug, Clone)]
struct Message {
    pub msg: String,
    pub file: String,
    pub line: Option<u64>,
    pub column: Option<u64>,
    pub span: (u64, u64),
}

#[derive(Debug, Clone, Default)]
struct Report {
    pub command: Vec<String>,
    pub root_directory: String,
    pub errors: Vec<Message>,
    pub warnings: Vec<Message>,
}

#[derive(Debug, Default)]
struct RegexMatcher {
    pat_error: Vec<Regex>,
    pat_warning: Vec<Regex>,
}

impl RegexMatcher {
    pub fn new() -> Self {
        Self { ..Default::default() }
    }

    pub fn add(&mut self, patterns: (&[&str], &[&str])) -> anyhow::Result<()> {
        // TODO limit size on the regexes

        for pat in patterns.0 {
            self.pat_error.push(
                Regex::new(pat).with_context(|| format!("Failed to compile pattern '{:?}'", pat))?
            );
        }

        for pat in patterns.1 {
            self.pat_warning.push(
                Regex::new(pat).with_context(|| format!("Failed to compile pattern '{:?}'", pat))?
            );
        }

        Ok(())
    }

    // pub fn find_all(&self, input: &str) {
    //
    // }
}


fn main() -> prelude::Result<()> {
    let args = cli::Cli::parse();

    // TODO maybe add --quiet flag to silence this
    eprint!("Executing");
    for i in &args.command {
        eprint!(" {:?}", i);
    }
    eprintln!();

    // TODO print which groups are being loaded
    // eprintln!()

    // TODO option to use stderr instead of stdout
    let mut handle = Command::new(args.command.first().unwrap())
        .args(args.command.iter().skip(1))
        .stdout(Stdio::piped())
        .spawn()
        .with_context(|| format!("Failed to execute command"))?;

    let stdout = handle.stdout.take().unwrap();

    let reader = BufReader::new(stdout);

    // preallocate the string, for the two lines
    let mut string = String::new();
    string.reserve(512);

    // TODO also check type_error / type_warning to set it only if some syntax is matched
    let re = regex::Regex::new(r#"(?<type>error|warning): (?<msg>.+)\n *--> *(?<file>.+):(?<line>\d+):(?<col>\d+)"#)?;

    let mut iter = reader.lines().peekable();
    while let Some(curr) = iter.next() {
        string = curr.as_ref().unwrap().clone();

        // add the second line to the string so regex can work over the two lines
        match iter.peek() {
            Some(x) => {
                string += "\n";
                string += x.as_ref().unwrap();
            },
            // do nothing as there is no second line
            None => {},
        };

        // print the line but to stderr
        eprintln!("{}", curr.unwrap());

        if let Some(m) = re.captures(&string) {
            println!("\n\n--- MATCH {:#?} ---\n\n", m);
        }
    }

    Ok(())
}
