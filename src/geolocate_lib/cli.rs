use clap::CommandFactory;

use crate::geolocate_lib::parser::CommandLineArguments;

pub fn build_cli() -> clap::Command {
    CommandLineArguments::command()
}
