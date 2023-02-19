#![allow(
    unused_variables,
    unused_mut,
    unused_imports,
    dead_code,
    unused_assignments
)]

use std::{
    fs::read_to_string,
    net::{AddrParseError, IpAddr},
    process::Command
};

use clap::Parser;
use geolocate::{
    ApiKeyStore, CommandLineArguments, ExclusiveConfigArgument,
    ExclusiveGeolocationArgument, Geolocation, GeolocationBuildError,
    MutualExclusivity, Provider, Subcommands
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
                    }

                    ExclusiveGeolocationArgument::File => {
                        let geolocation = Geolocation::try_new_from_file(
                            arguments.file.unwrap(),
                            Provider::Ip2Location,
                            store,
                            client
                        )?;
                    }
                },

                Err(err) => {
                    eprintln!("{}", err.to_string());
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
                    }
                    ExclusiveGeolocationArgument::File => {
                        let geolocation = Geolocation::try_new_from_file(
                            arguments.file.unwrap(),
                            Provider::IpGeolocation,
                            store,
                            client
                        )?;
                    }
                },

                Err(err) => {
                    eprintln!("{}", err.to_string());
                }
            }
        }

        Subcommands::Config(arguments) => match arguments.check_exclusivity() {
            Ok(value) => match value {
                ExclusiveConfigArgument::Edit => {
                    Command::new("vim")
                        .arg(
                            confy::get_configuration_file_path(
                                "geolocate",
                                None
                            )
                            .unwrap()
                        )
                        .spawn()
                        .unwrap()
                        .wait()
                        .unwrap();
                }

                ExclusiveConfigArgument::Show => {
                    println!(
                        "{}",
                        read_to_string(
                            confy::get_configuration_file_path(
                                "geolocate",
                                None
                            )
                            .unwrap()
                        )
                        .unwrap()
                    );
                }
            },
            Err(err) => {
                eprintln!("{}", err);
            }
        }
    };

    anyhow::Ok(())
}
