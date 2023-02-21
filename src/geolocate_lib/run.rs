use clap::Parser;

use super::completions::generate_shell_completions;
use super::config::operate_on_config_file;
use super::ip2location::response_from_ip2location;
use super::ipgeolocation::response_from_ipgeolocation;
use super::loader;
use super::parser::{Command, CommandLineArguments};

pub async fn run() -> anyhow::Result<()> {
    let args = CommandLineArguments::parse();
    let store = match loader::load_configuration_file() {
        Ok(store) => store,
        Err(_) => loader::open_configuration_file_with_editor()?,
    };

    match args.command {
        Command::Ip2location(arguments) => {
            response_from_ip2location(arguments, &store).await
        }

        Command::Ipgeolocation(arguments) => {
            response_from_ipgeolocation(arguments, &store).await
        }

        Command::Config(arguments) => operate_on_config_file(arguments, store),

        Command::Completions(shell) => generate_shell_completions(shell),
    }
}
