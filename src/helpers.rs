use std::{fs::read_to_string, process::Command};

use crate::{
    ApiKeyStore, ExclusiveConfigArgument, ExclusiveGeolocationArgument,
    Geolocation, GeolocationInput, MutualExclusivity, Provider
};

pub async fn fetch_from_provider<T>(
    mut arguments: T,
    store: ApiKeyStore,
    client: reqwest::Client
) -> anyhow::Result<()>
where
    T: MutualExclusivity<ExclusiveValue = ExclusiveGeolocationArgument>
        + GeolocationInput
{
    match arguments.check_exclusivity() {
        Ok(value) => match value {
            ExclusiveGeolocationArgument::IpAddresses => {
                let geolocation = Geolocation::try_new(
                    arguments.addrs().unwrap(),
                    Provider::Ip2Location,
                    store,
                    client
                )?;
                let data = geolocation.fetch().await?;
                println!("{}", serde_json::to_string_pretty(&data)?);
                anyhow::Ok(())
            }

            ExclusiveGeolocationArgument::File => {
                let geolocation = Geolocation::try_new_from_file(
                    arguments.file().unwrap(),
                    Provider::Ip2Location,
                    store,
                    client
                )?;
                let data = geolocation.fetch().await?;
                println!("{}", serde_json::to_string_pretty(&data)?);
                anyhow::Ok(())
            }
        },

        Err(err) => {
            eprintln!("{}", err);
            anyhow::Ok(())
        }
    }
}

pub fn read_or_modify_configuration<T>(arguments: T) -> anyhow::Result<()>
where
    T: MutualExclusivity<ExclusiveValue = ExclusiveConfigArgument>
{
    match arguments.check_exclusivity() {
        Ok(value) => match value {
            ExclusiveConfigArgument::Edit => {
                let editor =
                    std::env::var("EDITOR").unwrap_or("nano".to_string());
                let path =
                    confy::get_configuration_file_path("geolocate", None)?;
                Command::new(editor).arg(path).spawn()?.wait()?;
                anyhow::Ok(())
            }

            ExclusiveConfigArgument::Show => {
                let config_file_path =
                    confy::get_configuration_file_path("geolocate", None)?;

                let content = read_to_string(config_file_path)?;

                println!("{}", content.trim());
                anyhow::Ok(())
            }
        },
        Err(err) => {
            eprintln!("{}", err);
            anyhow::Ok(())
        }
    }
}
