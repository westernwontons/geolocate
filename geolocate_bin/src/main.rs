use geolocate_lib::{
    completions::generate_shell_completions,
    helpers::{
        fetch_from_provider, load_configuration, read_or_modify_configuration,
    },
    parser::{CommandLineArguments, Subcommands},
    reexport::Parser,
};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let args = CommandLineArguments::parse();
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
            read_or_modify_configuration(arguments)
        }

        Subcommands::Completions(shell) => generate_shell_completions(shell),
    }?;

    anyhow::Ok(())
}
