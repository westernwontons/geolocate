use geolocate_lib::{
    completions::generate_shell_completions,
    geolocation::Provider,
    helpers::{
        fetch_from_provider, load_configuration, read_or_modify_configuration,
    },
    parser::{CommandLineArguments, Subcommands},
    Parser,
};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let args = CommandLineArguments::parse();
    let store = load_configuration()?;

    match args.command {
        Subcommands::Ip2location(arguments) => fetch_from_provider::<
            _,
            serde_json::Map<String, serde_json::Value>,
        >(
            arguments,
            store,
            Provider::Ip2Location,
        )
        .await,

        Subcommands::Ipgeolocation(arguments) => fetch_from_provider::<
            _,
            serde_json::Map<String, serde_json::Value>,
        >(
            arguments,
            store,
            Provider::IpGeolocation,
        )
        .await,

        Subcommands::Config(arguments) => {
            read_or_modify_configuration(arguments)
        }

        Subcommands::Completions(shell) => {
            let path = generate_shell_completions(shell)?;
            println!("Generated shell completions to: {}", path.display());
            anyhow::Ok(())
        }
    }?;

    anyhow::Ok(())
}
