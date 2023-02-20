use std::{fs::read_to_string, net::IpAddr, path::PathBuf, process::Command};

use anyhow::Context;
use serde::{de::DeserializeOwned, Serialize};

use crate::{
    config::ApiKeyStore,
    geolocation::{Geolocation, Provider},
    parser::{ExclusiveConfigArgument, ExclusiveGeolocationArgument},
    traits::{GeolocationInput, MutualExclusivity},
};

/// Fetch geolocation data from a provider. The provider is determined
/// with the help of the Provider enum, which is a required parameter
/// for this function.
pub async fn fetch_from_provider<T, E>(
    mut arguments: T,
    store: ApiKeyStore,
    provider: Provider,
) -> anyhow::Result<()>
where
    T: MutualExclusivity<ExclusiveValue = ExclusiveGeolocationArgument>
        + GeolocationInput,
    E: Serialize + DeserializeOwned + Send + 'static,
{
    let api_key = store.get_provider_token(&provider)?;

    match arguments.check_exclusivity() {
        Ok(value) => match value {
            ExclusiveGeolocationArgument::IpAddresses => {
                let mut geolocation =
                    Geolocation::new(arguments.addrs().unwrap(), api_key);
                let data = geolocation.fetch::<E>(provider).await?;
                println!("{}", serde_json::to_string_pretty(&data)?);
                anyhow::Ok(())
            }

            ExclusiveGeolocationArgument::File => {
                let ip_addrs =
                    read_ip_addresses_from_file(arguments.file().unwrap())?;

                if let Some(Err(ip_addr_error)) =
                    ip_addrs.iter().find(|item| item.is_err())
                {
                    anyhow::bail!("{}", ip_addr_error);
                }
                let ip_addrs =
                    ip_addrs.into_iter().map(Result::unwrap).collect();
                let mut geolocation = Geolocation::new(ip_addrs, api_key);
                let data = geolocation.fetch::<E>(provider).await?;
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

/// Read or modify the configuration file where the API keys are stored
pub fn read_or_modify_configuration<T>(arguments: T) -> anyhow::Result<()>
where
    T: MutualExclusivity<ExclusiveValue = ExclusiveConfigArgument>,
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

/// Read a list of IP addresses from a file. IP addresses have to be
/// separated by newline. Each of them will be parsed, but note that the
/// caller will have to handle any errors.
pub fn read_ip_addresses_from_file(
    file: PathBuf,
) -> anyhow::Result<Vec<anyhow::Result<IpAddr>>> {
    let ip_addresses = read_to_string(file)?
        .split_terminator('\n')
        .enumerate()
        .map(|(index, item)| {
            item.parse::<IpAddr>().with_context(|| {
                format!("IP address at line {} is invalid", index)
            })
        })
        .collect::<Vec<anyhow::Result<IpAddr>>>();

    anyhow::Ok(ip_addresses)
}

/// Helper function to fetch geolocation data.
/// Initiates requests concurrently.
pub async fn fetch_many<T>(
    urls: Vec<String>,
    client: &reqwest::Client,
) -> anyhow::Result<Vec<T>>
where
    T: Serialize + DeserializeOwned + Send + 'static,
{
    let urls_len = urls.len();
    let futures = urls
        .into_iter()
        .map(|url| {
            let client = client.clone();

            tokio::spawn(async move {
                client.get(url).send().await?.json::<T>().await
            })
        })
        .collect::<Vec<_>>();
    let mut results = Vec::with_capacity(urls_len);
    for future in futures {
        results.push(future.await??);
    }
    anyhow::Ok(results)
}

pub fn load_configuration() -> anyhow::Result<ApiKeyStore> {
    confy::load::<ApiKeyStore>("geolocate", None)
        .map_err(|err| anyhow::anyhow!("{}", err))
}
