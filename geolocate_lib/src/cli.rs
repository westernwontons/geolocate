use clap::CommandFactory;

use crate::parser::CommandLineArguments;

pub fn build_cli() -> clap::Command {
    CommandLineArguments::command()
}
