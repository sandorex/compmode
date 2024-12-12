mod cli;
mod patterns;
mod parser;
mod report;
mod message;

use message::Message;
use patterns::pick_group;
use regex::Regex;
use report::Report;
use anyhow::{Context, Result, anyhow};
use clap::Parser;
use std::{io::{BufRead, BufReader}, process::{Command, ExitCode, Stdio}};

fn parse_n_print<T>(_args: &cli::Cli,
                    patterns: &Vec<Regex>,
                    childio: T,
                    report: &mut Report
                    ) -> Result<()> where T: std::io::Read {
    let reader = BufReader::new(childio);

    let mut string = String::new();
    string.reserve(512);

    let mut position: usize = 0;

    // NOTE this whole mess is to allow regex to parse two lines at a time as some executors split
    // the messages (ex. cargo)
    let mut iter = reader.lines().peekable();
    while let Some(curr) = iter.next() {
        /// Length of the newline sequence in bytes (windows is \r\n while linux \n)
        const NEWLINE_LEN: usize = if cfg!(windows) { 2 } else { 1 };

        string = curr.as_ref().unwrap().clone();
        let curr_length = string.len() + NEWLINE_LEN;
        let mut total_length = curr_length;

        match iter.peek() {
            Some(x) => {
                // concat the second line so regex can work over both
                string += "\n";
                string += x.as_ref().unwrap();

                // add the length of the line to total
                total_length += x.as_ref().unwrap().len() + NEWLINE_LEN;
            },
            // do nothing as there is no second line
            None => {},
        };

        // print the line but to stderr
        eprintln!("{}", curr.as_ref().unwrap());

        if let Some(capture) = parser::capture_first(patterns, &string) {
            let mut msg = match TryInto::<Message>::try_into(&capture) {
                Ok(x) => x,
                Err(x) => {
                    // TODO give more information
                    eprintln!("[compmode]: Error invalid capture: {}", x);
                    continue
                },
            };

            // correct the span
            msg.span = (
                msg.span.0 + position,
                msg.span.1 + position
            );

            // TODO check if this works on windows with '\r\n'!
            // if the capture has a newline in its whole span then it probably takes both lines
            if capture.get(0).unwrap().as_str().contains('\n') {
                position += total_length;

                // skip next line as it was already consumed
                iter.next();
            } else {
                position += curr_length;
            }

            report.messages.push(msg);
        }
    }

    Ok(())
}

fn execute(args: cli::Cli) -> Result<i32> {
    let patterns = {
        let groups = pick_group(&args.regex_group, args.command.first().unwrap())?;

        let mut patterns: Vec<Regex> = vec![];

        // compile all patterns
        for group in groups {
            for pat in group.iter() {
                patterns.push(Regex::new(pat)
                    .with_context(|| anyhow!("Could not compile {:?}", pat))?);
            }
        }

        patterns
    };

    if ! args.quiet {
        eprint!("Executing");
        for i in &args.command {
            eprint!(" {:?}", i);
        }
        eprintln!();
        eprintln!("------------------------------------");
    }

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

    parse_n_print(&args, &patterns, child.stdout.take().unwrap(), &mut report)?;

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

        for (name, patterns) in patterns::GROUPS.iter().zip(patterns::ALL.iter()) {
            println!("{}:", name);

            for pat in patterns.iter() {
                println!("  {}", pat);
            }
        }
        println!();

        return ExitCode::SUCCESS;
    }

    let result = execute(args);

    match result {
        // turn i32 into u8..
        Ok(x) => ExitCode::from(TryInto::<u8>::try_into(x).unwrap_or(1)),
        Err(e) => {
            eprintln!("{}", e);

            ExitCode::FAILURE
        },
    }
}
