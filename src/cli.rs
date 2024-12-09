use clap::{Parser, ValueEnum};

pub const FULL_VERSION: &str = concat!(env!("CARGO_PKG_VERSION"), "-", env!("VERGEN_GIT_DESCRIBE"), " (", env!("VERGEN_GIT_BRANCH"), ")");

pub const AFTER_HELP: &str = "If you wish to support or improve compmode go to
    https://github.com/sandorex/compmode
";

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Debug, ValueEnum)]
pub enum Format {
    /// Format meant for testing
    Debug,
    JSON,

    /// Null terminated mode where only messages are printed, one per line
    NULL,
}

impl ToString for Format {
    fn to_string(&self) -> String {
        format!("{:?}", self).to_lowercase()
    }
}

/// Standalone utility to imitate emacs amazing compile-mode
///
/// Intended for use through plugins which glue it into the editor
#[derive(Parser, Debug)]
#[command(name = env!("CARGO_PKG_NAME"), author, version = FULL_VERSION, about, after_help = AFTER_HELP)]
pub struct Cli {
    /// Use specific API version, used to provide backward compatibility, '0' means latest
    #[arg(long, default_value_t = 0)]
    pub api_version: u16,

    /// Output data in specific format, default is debug
    #[arg(short, long, default_value_t = Format::Debug)]
    pub format: Format,

    /// Explicitly use a specific regex group (ex. 'gcc' group for all flavours of gcc compiler)
    ///
    /// If 'auto' then autodetect from the command, if unsuccessful use all of them
    #[arg(short, long, default_value = "auto")]
    pub regex_group: String,

    // TODO but do not require the command
    // /// Lists all regexes builtin, does not do anything else
    #[arg(long)]
    pub list_regex: bool,

    /// Command to execute
    #[arg(last = true, required_if_eq("list_regex", "false"))]
    pub command: Vec<String>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn verify_cli() {
        use clap::CommandFactory;
        Cli::command().debug_assert()
    }
}
