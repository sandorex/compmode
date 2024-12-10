use crate::message::Message;
use crate::patterns::PatternList;
use anyhow::{anyhow, Result, Context};
use regex::Regex;

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

