use regex::{Captures, Regex};

/// Returns capture from first pattern that matches input
pub fn capture_first<'a>(patterns: &Vec<Regex>, input: &'a str) -> Option<Captures<'a>> {
    for pat in patterns {
        if let Some(capture) = pat.captures(input) {
            return Some(capture);
        }
    }

    None
}

