use geolocate_lib::{
    completions::generate_shell_completions,
    helpers::{
        fetch_from_provider, load_configuration, open_config_file_with_editor,
        print_configuration_file_path, read_or_modify_configuration,
    },
    parser::{CommandLineArguments, Subcommands},
    reexport::{ConfyError, Parser},
};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let args = CommandLineArguments::parse();
    if let Err(error) = load_configuration() {
        match error {
            ConfyError::BadTomlData(_) => open_config_file_with_editor()?,
            err @ _ => anyhow::bail!("{}", err),
        }
    };
    let store = load_configuration()?;

    match args.command {
        Subcommands::Ip2location(arguments) => fetch_from_provider::<
            _,
            serde_json::Map<String, serde_json::Value>,
        >(arguments, store)
        .await,

        Subcommands::Ipgeolocation(arguments) => fetch_from_provider::<
            _,
            serde_json::Map<String, serde_json::Value>,
        >(arguments, store)
        .await,

        Subcommands::Config(arguments) => {
            if arguments.print_path()? {
                return anyhow::Ok(print_configuration_file_path()?);
            }
            read_or_modify_configuration(arguments)
        }

        Subcommands::Completions(shell) => generate_shell_completions(shell),
    }?;

    anyhow::Ok(())
}
