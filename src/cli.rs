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

    /// CSV but with separator being \0, header and messages are printed
    NullSep,
}

impl ToString for Format {
    fn to_string(&self) -> String {
        format!("{:?}", self).to_lowercase()
    }
}

// TODO add option to buffer the input (with memory limit) and send using json

/// Standalone utility to imitate emacs amazing compile-mode
///
/// Intended for use through plugins which glue it into the editor
#[derive(Parser, Debug)]
#[command(name = env!("CARGO_PKG_NAME"), author, version = FULL_VERSION, about, after_help = AFTER_HELP)]
pub struct Cli {
    /// Output data in specific format, default is debug
    #[arg(short, long, default_value_t = Format::Debug)]
    pub format: Format,

    /// Does not print any additional text (the executor stdout is still printed)
    #[arg(short, long)]
    pub quiet: bool,

    /// Explicitly use a specific regex group (ex. 'gcc' group for all flavours of gcc compiler)
    ///
    /// Special groups: 'all' - use all regex groups at once (could be bit slower)
    ///                 'auto' - automatically detect which groups to use based on the command
    #[arg(short, long, default_value = "auto")]
    pub regex_group: String,

    // /// Lists all regex groups, then terminates the program
    // #[arg(long, default_value_t = false)]
    // pub list_regex: bool,

    /// Command to execute
    #[arg(last = true, required = true, num_args = 1..)]
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
