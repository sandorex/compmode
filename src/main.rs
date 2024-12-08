mod cli;

use clap::Parser;

pub mod prelude {
    pub use anyhow::{anyhow, Context as AnyhowContext, Result};
}

fn main() {
    let args = cli::Cli::parse();

    dbg!(args);
}
