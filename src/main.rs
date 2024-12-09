mod cli;
mod patterns;
mod parser;

use cli::Format;
use parser::{Message, MessageParser};
use anyhow::{Context, Result, anyhow};
use clap::Parser;
use serde::Serialize;
use std::{io::{BufRead, BufReader}, process::{Command, ExitCode, Stdio}};

/// Length of the newline sequence in bytes (windows is \r\n while linux \n)
const NEWLINE_LEN: usize = if cfg!(windows) { 2 } else { 1 };

#[derive(Debug, Clone, Serialize)]
pub struct Report {
    pub command: Vec<String>,
    pub root_directory: String,
    pub messages: Vec<Message>,
    pub exit_code: i32,
}

impl Report {
    pub fn format_with(&self, format: Format, _version: u16) -> Result<String> {
        let mut output = String::new();

        match format {
            cli::Format::Debug => {
                output += format!("{:#?}", self).as_str();
            },
            cli::Format::JSON => {
                let serialized = serde_json::to_string(self)
                    .with_context(|| anyhow!("Failed to serialize report"))?;

                output += format!("{}", serialized).as_str();
            },
            cli::Format::NULL => {
                for msg in &self.messages {
                    output += format!(
                        "\n{}\0{}\0{}\0",
                        if msg.is_error { "1" } else { "0" },
                        msg.msg,
                        msg.file,
                    ).as_str();

                    if let Some(line) = msg.line {
                        output += line.to_string().as_str();
                    }

                    output += "\0";

                    if let Some(col) = msg.column {
                        output += col.to_string().as_str();
                    }

                }
            }
        }

        Ok(output)
    }
}

fn execute(args: cli::Cli) -> Result<i32> {
    if ! args.quiet {
        eprint!("Executing");
        for i in &args.command {
            eprint!(" {:?}", i);
        }
        eprintln!();
        eprintln!("------------------------------------");
    }

    // TODO print which groups are being loaded
    // eprintln!()

    let mut child = Command::new(args.command.first().unwrap())
        .args(args.command.iter().skip(1))
        .stdout(Stdio::piped())
        .spawn()
        .with_context(|| format!("Failed to execute command"))?;

    let mut parser = MessageParser::new(&vec![patterns::cargo::PATTERN])?;

    let mut report = Report {
        command: args.command.clone(),
        root_directory: std::env::current_dir().unwrap().to_string_lossy().to_string(),
        messages: vec![],
        exit_code: 0,
    };

    let reader = BufReader::new(child.stdout.take().unwrap());

    let mut string = String::new();
    string.reserve(512);

    // NOTE this whole mess is to allow regex to parse two lines at a time as some executors split
    // the messages (ex. cargo)
    let mut iter = reader.lines().peekable();
    while let Some(curr) = iter.next() {
        string = curr.as_ref().unwrap().clone();
        let length = curr.as_ref().unwrap().len() + NEWLINE_LEN;

        match iter.peek() {
            Some(x) => {
                // concat the second line so regex can work over both
                string += "\n";
                string += x.as_ref().unwrap();
            },
            // do nothing as there is no second line
            None => {},
        };

        // print the line but to stderr
        eprintln!("{}", curr.unwrap());

        report.messages.extend(parser.parse(&string)?);

        // tell the parser that stream has advanced so span is correct
        parser.advance(length);
    }

    let exit_result = child.wait()?;

    if ! args.quiet {
        eprintln!("------------------------------------");
        eprintln!("Child process {}", exit_result);
    }

    // get exit code or fallback as -1 if terminated by a signal
    report.exit_code = exit_result.code().unwrap_or(-1);

    println!("{}", report.format_with(args.format, args.api_version)?);

    // return same exit code
    Ok(report.exit_code)
}

fn main() -> ExitCode {
    let args = cli::Cli::parse();

    if args.list_regex {
        println!("Printing all regex patterns");

        for (name, pat) in patterns::GROUPS {
            println!("{}: {:#?}", name, pat);
        }
        println!();

        return ExitCode::SUCCESS;
    }

    let result = execute(args);

    // TODO maybe option to set exit code when error happens so it is detected
    match result {
        // turn i32 into u8..
        Ok(x) => ExitCode::from(TryInto::<u8>::try_into(x).unwrap_or(1)),
        Err(e) => {
            eprintln!("{}", e);

            ExitCode::FAILURE
        },
    }
}
