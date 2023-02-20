use clap::Parser;
use geolocate::{
    fetch_from_provider, read_or_modify_configuration, ApiKeyStore,
    CommandLineArguments, Provider, Subcommands,
};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let args = CommandLineArguments::parse();
    let store = confy::load::<ApiKeyStore>("geolocate", None)?;

    match args.commands {
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
    }?;

    anyhow::Ok(())
}
