mod cli;
mod patterns;
mod parser;

use parser::{Message, MessageParser};
use anyhow::{Context, Result};
use clap::Parser;
use std::{io::{BufRead, BufReader}, process::{Command, Stdio}};

/// Length of the newline sequence in bytes (windows is \r\n while linux \n)
const NEWLINE_LEN: usize = if cfg!(windows) { 2 } else { 1 };

#[derive(Debug, Clone)]
pub struct Report {
    pub command: Vec<String>,
    pub root_directory: String,
    pub messages: Vec<Message>,
    // pub exit_code: u8,
}

fn execute(args: cli::Cli) -> Result<()> {
    // TODO maybe add --quiet flag to silence this
    eprint!("Executing");
    for i in &args.command {
        eprint!(" {:?}", i);
    }
    eprintln!();

    // TODO print which groups are being loaded
    // eprintln!()

    // TODO option to use stderr instead of stdout
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

        parser.advance(length);
    }

    // TODO print exit status to stderr
    let exit_result = child.wait();
    println!("exit: {:?}", exit_result);

    match args.format {
        cli::Format::Debug => {
            println!("--- DEBUG REPORT ---");
            println!("{:#?}", report);
            println!("--- DEBUG REPORT ---");
        },
        cli::Format::JSON => {
            todo!();
        },
        cli::Format::NULL => {
            for msg in report.messages {
                print!(
                    "{}\0{}\0{}\0",
                    if msg.is_error { "1" } else { "0" },
                    msg.msg,
                    msg.file,
                );

                if let Some(line) = msg.line {
                    print!("{}", line);
                }

                print!("\0");

                if let Some(col) = msg.column {
                    print!("{}", col);
                }

                println!();
            }
        }
    }

    // TODO return same exit code as the command ran

    Ok(())
}

fn main() -> Result<()> {
    let args = cli::Cli::parse();

    if args.list_regex {
        println!("Printing all regex patterns");

        for (name, pat) in patterns::GROUPS {
            println!("{}: {:#?}", name, pat);
        }
        println!();

        return Ok(());
    }

    let result = execute(args);

    // TODO filter out and return same value
    result
}
