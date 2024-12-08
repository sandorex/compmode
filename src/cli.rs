use clap::Parser;

pub const FULL_VERSION: &str = concat!(env!("CARGO_PKG_VERSION"), "-", env!("VERGEN_GIT_DESCRIBE"), " (", env!("VERGEN_GIT_BRANCH"), ")");

pub const AFTER_HELP: &str = "If you wish to add support for another editor or executor please go to
    https://github.com/sandorex/compmode-plugins
";

/// Standalone utility to imitate emacs amazing compile-mode
///
/// Intended for use through plugins which glue it into the editor
#[derive(Parser, Debug)]
#[command(name = env!("CARGO_PKG_NAME"), author, version = FULL_VERSION, about, after_help = AFTER_HELP)]
pub struct Cli {
    /// Output data in JSON format
    #[arg(short, long)]
    pub json: bool,

    // TODO but do not require the command
    // /// Lists all regexes builtin, does not do anything else
    // #[arg(long)]
    // pub list_regexes: bool,

    /// Command to execute
    #[arg(last = true, required = true)]
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
