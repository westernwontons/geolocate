use clap::Parser;
use geolocate::{
    fetch_from_provider, read_or_modify_configuration, ApiKeyStore,
    CommandLineArguments, Subcommands
};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let args = CommandLineArguments::parse();
    let store = confy::load::<ApiKeyStore>("geolocate", None)?;
    let client = reqwest::Client::new();

    match args.commands {
        Subcommands::Ip2location(arguments) => {
            fetch_from_provider(arguments, store, client).await
        }

        Subcommands::Ipgeolocation(arguments) => {
            fetch_from_provider(arguments, store, client).await
        }

        Subcommands::Config(arguments) => {
            read_or_modify_configuration(arguments)
        }
    }?;

    anyhow::Ok(())
}
