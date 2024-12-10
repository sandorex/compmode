use serde::Serialize;
use regex::Captures;

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

impl PartialEq for Message {
    fn eq(&self, other: &Self) -> bool {
        self.span == other.span
            && self.line == other.line
            && self.column == other.column
    }
}

impl TryFrom<&Captures<'_>> for Message {
    type Error = anyhow::Error;

    fn try_from(captures: &Captures<'_>) -> Result<Self, Self::Error> {
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
