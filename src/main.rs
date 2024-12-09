mod cli;
mod patterns;
mod parser;
mod report;

use parser::MessageParser;
use patterns::Pattern;
use report::Report;
use anyhow::{Context, Result, anyhow};
use clap::Parser;
use std::{io::{BufRead, BufReader}, process::{Command, ExitCode, Stdio}};

/// Length of the newline sequence in bytes (windows is \r\n while linux \n)
const NEWLINE_LEN: usize = if cfg!(windows) { 2 } else { 1 };

fn pick_group(args: &cli::Cli) -> Result<MessageParser> {
    let mut groups: Vec<&Pattern> = vec![];

    match args.regex_group.as_str() {
        "all" => groups.extend(patterns::GROUPS),
        "auto" => todo!(),
        // choose exact group
        x => {
            for group in patterns::GROUPS {
                if group.0.to_lowercase() == x {
                    groups.push(group);
                    break;
                }
            }

            if groups.is_empty() {
                return Err(anyhow!("Could not find regex group {:?}", x));
            }
        }
    }

    // if ! args.quiet {
    //     eprint!("Regex groups used:" );
    //     for (name, _) in &groups {
    //         eprint!(" {name}");
    //     }
    //     eprintln!();
    // }

    Ok(MessageParser::new(&groups)?)
}

fn execute(args: cli::Cli) -> Result<i32> {
    let mut parser = pick_group(&args)?;

    if ! args.quiet {
        eprint!("Executing");
        for i in &args.command {
            eprint!(" {:?}", i);
        }
        eprintln!();
        eprintln!("------------------------------------");
    }

    // let mut parser = MessageParser::new(&vec![patterns::cargo::PATTERN])?;

    let mut child = Command::new(args.command.first().unwrap())
        .args(args.command.iter().skip(1))
.stdout(Stdio::piped())
        .spawn()
        .with_context(|| format!("Failed to execute command"))?;

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

    println!("{}", report.format_with(args.format)?);

    // return same exit code
    Ok(report.exit_code)
}

fn main() -> ExitCode {
    let args = cli::Cli::parse();

    if args.list_regex {
        println!("Listing all regex groups");

        for (name, pat) in patterns::GROUPS {
            println!("{}:", name);

            for i in pat.iter() {
                println!("  {}", i);
            }
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
