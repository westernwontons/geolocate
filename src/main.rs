use std::{fs::read_to_string, process::Command};

use clap::Parser;
use geolocate::{
    ApiKeyStore, CommandLineArguments, ExclusiveConfigArgument,
    ExclusiveGeolocationArgument, Geolocation, MutualExclusivity, Provider,
    Subcommands
};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let args: CommandLineArguments = CommandLineArguments::parse();
    let store = confy::load::<ApiKeyStore>("geolocate", None)?;
    let client = reqwest::Client::new();

    match args.commands {
        Subcommands::Ip2location(arguments) => {
            match arguments.check_exclusivity() {
                Ok(value) => match value {
                    ExclusiveGeolocationArgument::IpAddresses => {
                        let geolocation = Geolocation::try_new(
                            arguments.addrs.unwrap(),
                            Provider::Ip2Location,
                            store,
                            client
                        )?;
                        let data = geolocation.fetch().await?;
                        println!("{}", serde_json::to_string_pretty(&data)?)
                    }

                    ExclusiveGeolocationArgument::File => {
                        let geolocation = Geolocation::try_new_from_file(
                            arguments.file.unwrap(),
                            Provider::Ip2Location,
                            store,
                            client
                        )?;
                        let data = geolocation.fetch().await?;
                        println!("{}", serde_json::to_string_pretty(&data)?)
                    }
                },

                Err(err) => {
                    eprintln!("{}", err);
                }
            }
        }

        Subcommands::Ipgeolocation(arguments) => {
            match arguments.check_exclusivity() {
                Ok(value) => match value {
                    ExclusiveGeolocationArgument::IpAddresses => {
                        let geolocation = Geolocation::try_new(
                            arguments.addrs.unwrap(),
                            Provider::IpGeolocation,
                            store,
                            client
                        )?;
                        let data = geolocation.fetch().await?;
                        println!("{}", serde_json::to_string_pretty(&data)?);
                    }
                    ExclusiveGeolocationArgument::File => {
                        let geolocation = Geolocation::try_new_from_file(
                            arguments.file.unwrap(),
                            Provider::IpGeolocation,
                            store,
                            client
                        )?;
                        let data = geolocation.fetch().await?;
                        println!("{}", serde_json::to_string_pretty(&data)?)
                    }
                },

                Err(err) => {
                    eprintln!("{}", err);
                }
            }
        }

        Subcommands::Config(arguments) => match arguments.check_exclusivity() {
            Ok(value) => match value {
                ExclusiveConfigArgument::Edit => {
                    let editor =
                        std::env::var("EDITOR").unwrap_or("nano".to_string());
                    let path =
                        confy::get_configuration_file_path("geolocate", None)?;
                    Command::new(editor).arg(path).spawn()?.wait()?;
                }

                ExclusiveConfigArgument::Show => {
                    let config_file_path =
                        confy::get_configuration_file_path("geolocate", None)?;

                    let content = read_to_string(config_file_path)?;

                    println!("{}", content.trim());
                }
            },
            Err(err) => {
                eprintln!("{}", err);
            }
        }
    };

    anyhow::Ok(())
}
