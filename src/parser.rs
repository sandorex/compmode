use regex::Regex;
use anyhow::{Context, anyhow, Result};
use serde::Serialize;

use crate::patterns::PatternList;

// TODO add serialization
/// Contains one message with its information, used for both errors and warnings
#[derive(Debug, Clone, Serialize)]
pub struct Message {
    pub is_error: bool,
    pub msg: String,
    pub file: String,
    pub line: Option<usize>,
    pub column: Option<usize>,
    pub span: (usize, usize),
}

// NOTE this is to filter out same messages
impl PartialEq for Message {
    fn eq(&self, other: &Self) -> bool {
        self.span == other.span
            && self.line == other.line
            && self.column == other.column
    }
}

impl TryFrom<&regex::Captures<'_>> for Message {
    type Error = anyhow::Error;

    fn try_from(captures: &regex::Captures<'_>) -> std::result::Result<Self, Self::Error> {
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

        // TODO has a lot of unwraps make nice anyhow errors!
        Ok(Message {
            is_error,
            msg: captures.name("msg").unwrap().as_str().to_string(),
            file: captures.name("file").unwrap().as_str().to_string(),
            line: captures.name("line").map(|x| x.as_str().parse::<usize>().unwrap()),
            column: captures.name("col").map(|x| x.as_str().parse::<usize>().unwrap()),
            span: {
                let m = captures.get(0).unwrap();

                (m.start(), m.end())
            },
        })
    }
}

pub struct MessageParser {
    position: usize,
    patterns: Vec<Regex>,
}

impl MessageParser {
    pub fn new(groups: &Vec<&PatternList>) -> Result<Self> {
        let mut patterns: Vec<Regex> = vec![];

        for group in groups {
            for pat in group.iter() {
                patterns.push(
                    Regex::new(pat)
                    .with_context(|| anyhow!("Failed to compile pattern {:?}", pat))?
                )
            }
        }

        Ok(Self {
            patterns,
            position: 0,
        })
    }

    pub fn parse(&self, input: &str) -> anyhow::Result<Vec<Message>> {
        let mut messages: Vec<Message> = vec![];

        for pat in &self.patterns {
            for captures in pat.captures_iter(input) {
                let mut msg = TryInto::<Message>::try_into(&captures)?;

                // correct the span
                msg.span = (
                    msg.span.0 + self.position,
                    msg.span.1 + self.position
                );

                messages.push(msg);
            }
        }

        Ok(messages)
    }

    pub fn advance(&mut self, amount: usize) {
        self.position += amount;
    }
}

