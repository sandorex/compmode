use crate::parser::Message;
use crate::cli::Format;
use anyhow::{anyhow, Result, Context};
use serde::Serialize;

#[derive(Debug, Clone, Serialize)]
pub struct Report {
    pub command: Vec<String>,
    pub root_directory: String,
    pub messages: Vec<Message>,
    pub exit_code: i32,
}

impl Report {
    /// Formats as csv with any separator
    fn format_csv(&self, separator: &str, output: &mut String) {
        for msg in &self.messages {
            *output += format!(
                "\n{}{separator}{}{separator}{}{separator}",
                if msg.is_error { "1" } else { "0" },
                msg.msg,
                msg.file,
            ).as_str();

            if let Some(line) = msg.line {
                *output += line.to_string().as_str();
            }

            *output += separator;

            if let Some(col) = msg.column {
                *output += col.to_string().as_str();
            }
        }
    }

    pub fn format_with(&self, format: Format) -> Result<String> {
        let mut output = String::new();

        match format {
            Format::Debug => {
                output += format!("{:#?}", self).as_str();
            },
            Format::JSON => {
                let serialized = serde_json::to_string(self)
                    .with_context(|| anyhow!("Failed to serialize report"))?;

                output += format!("{}", serialized).as_str();
            },
            Format::CSV => self.format_csv(",", &mut output),
            Format::NULL => self.format_csv("\0", &mut output),
        }

        Ok(output)
    }
}