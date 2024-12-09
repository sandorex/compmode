mod cli;
mod patterns;

use anyhow::Context;
use clap::Parser;
use regex::Regex;
use std::{io::{BufRead, BufReader}, process::{Child, Command, Stdio}};

pub mod prelude {
    pub use anyhow::{anyhow, Context as AnyhowContext, Result};
}

// TODO move these structs and add serialization
/// Contains one message with its information, used for both errors and warnings
#[derive(Debug, Clone)]
struct Message {
    pub is_error: bool,
    pub msg: String,
    pub file: String,
    pub line: Option<usize>,
    pub column: Option<usize>,
    pub span: (usize, usize),
}

#[derive(Debug, Clone)]
struct Report {
    pub command: Vec<String>,
    pub root_directory: String,
    pub messages: Vec<Message>,
    // pub exit_code: u8,
}

pub fn capture_messages(regexes: &Vec<Regex>, input: &str) -> Vec<Message> {
    let mut messages: Vec<Message> = vec![];

    for pat in regexes {
        let Some(captures) = pat.captures(input) else { continue };

        let is_error = {
            let r#type = match captures.name("type") {
                Some(x) => match x.as_str().to_lowercase().as_str() {
                    "error" => true,
                    _ => false,
                },
                None => false,
            };

            let type_error = captures.name("type_error").is_some();

            r#type || type_error
        };

        // TODO has a lot of unwraps
        messages.push(Message {
            is_error,
            msg: captures.name("msg").unwrap().as_str().to_string(),
            file: captures.name("file").unwrap().as_str().to_string(),
            line: captures.name("line").map(|x| x.as_str().parse::<usize>().unwrap()),
            column: captures.name("col").map(|x| x.as_str().parse::<usize>().unwrap()),
            span: {
                let m = captures.get(0).unwrap();

                (m.start(), m.end())
            },
        });
    }

    messages
}

fn print_and_parse_pipe(child: &mut Child, regexes: &Vec<Regex>, messages: &mut Vec<Message>) {
    let reader = BufReader::new(child.stdout.take().unwrap());

    // preallocate the string, for the two lines
    let mut string = String::new();
    string.reserve(512);

    // NOTE this is complicated cause i need to run regex on at 2 lines at time for some cases like
    // cargo, otherwise i wont be able to catch whole pattern
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

        messages.extend(capture_messages(&regexes, &string));
    }
}

fn main() -> prelude::Result<()> {
    let args = cli::Cli::parse();

    if args.list_regex {
        println!("Printing all regex patterns");

        for (name, pat) in patterns::GROUPS {
            println!("{}: {:#?}", name, pat);
        }
        println!();

        return Ok(());
    }

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

    // TODO use the actual groups
    // compile regexes
    let mut re: Vec<Regex> = vec![];
    for pat in patterns::cargo::PATTERN.1 {
        re.push(Regex::new(pat)?);
    }

    let mut report = Report {
        command: args.command.clone(),
        root_directory: std::env::current_dir().unwrap().to_string_lossy().to_string(),
        messages: vec![],
    };

    print_and_parse_pipe(&mut handle, &re, &mut report.messages);

    // TODO print exit status to stderr
    let exit_result = handle.wait();
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
