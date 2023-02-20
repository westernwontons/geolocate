use clap::Parser;
use confy::ConfyError;

use crate::{
    completions::generate_shell_completions,
    helpers::{
        fetch_from_provider, load_configuration,
        open_config_file_with_preferred_editor, print_configuration_file_path,
        read_or_modify_configuration,
    },
    parser::{Command, CommandLineArguments},
};

pub async fn run() -> anyhow::Result<()> {
    let args = CommandLineArguments::parse();
    if let Err(error) = load_configuration() {
        match error {
            ConfyError::BadTomlData(_) => {
                open_config_file_with_preferred_editor()?
            }
            err => anyhow::bail!("{}", err),
        }
    };
    let store = load_configuration()?;

    match args.command {
        Command::Ip2location(arguments) => fetch_from_provider::<
            _,
            serde_json::Map<String, serde_json::Value>,
        >(arguments, store)
        .await,

        Command::Ipgeolocation(arguments) => fetch_from_provider::<
            _,
            serde_json::Map<String, serde_json::Value>,
        >(arguments, store)
        .await,

        Command::Config(arguments) => {
            if arguments.print_path() {
                return print_configuration_file_path();
            }
            read_or_modify_configuration(arguments)
        }

        Command::Completions(shell) => generate_shell_completions(shell),
    }?;

    Ok(())
}
