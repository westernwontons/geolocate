use std::{fs::read_to_string, net::IpAddr, path::PathBuf, process::Command};

use anyhow::Context;
use confy::ConfyError;
use json_color::Colorizer;
use serde::{de::DeserializeOwned, Serialize};

use crate::{
    config::ApiKeyStore,
    geolocation::Geolocation,
    parser::{ExclusiveConfigArgument, ExclusiveGeolocationArgument},
    traits::{GeolocationInput, GeolocationProvider, MutualExclusivity},
};

/// Fetch geolocation data from a provider. The provider is determined
/// with the help of the Provider enum, which is a required parameter
/// for this function.
pub async fn fetch_from_provider<T, E>(
    mut arguments: T,
    store: ApiKeyStore,
) -> anyhow::Result<()>
where
    T: MutualExclusivity<ExclusiveValue = ExclusiveGeolocationArgument>
        + GeolocationInput
        + GeolocationProvider,
    E: Serialize + DeserializeOwned + Send + 'static,
{
    match arguments.check_exclusivity() {
        Ok(value) => match value {
            ExclusiveGeolocationArgument::IpAddresses => {
                let provider = arguments.provider();
                let api_key = store.get_provider_token(&provider)?;
                let mut geolocation =
                    Geolocation::new(arguments.addrs().unwrap(), api_key);
                let data = geolocation.fetch::<E>(provider).await?;
                colorize_json(data)
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
                let provider = arguments.provider();
                let api_key = store.get_provider_token(&provider)?;
                let mut geolocation = Geolocation::new(ip_addrs, api_key);
                let data = geolocation.fetch::<E>(provider).await?;
                colorize_json(data)
            }
        },

        Err(err) => {
            eprintln!("{}", err);
            Ok(())
        }
    }
}

pub fn colorize_json<E>(json_string: Vec<E>) -> anyhow::Result<()>
where
    E: Serialize,
{
    let colorizer = Colorizer::arbitrary();
    let json = serde_json::to_string_pretty(&json_string)?;
    println!("{}", colorizer.colorize_json_str(&json)?);
    Ok(())
}

/// Read or modify the configuration file where the API keys are stored
pub fn read_or_modify_configuration<T>(arguments: T) -> anyhow::Result<()>
where
    T: MutualExclusivity<ExclusiveValue = ExclusiveConfigArgument>,
{
    match arguments.check_exclusivity() {
        Ok(value) => match value {
            ExclusiveConfigArgument::Edit => {
                let path =
                    confy::get_configuration_file_path("geolocate", None)?;
                open_config_file_with_preferred_editor()?;
                match toml::from_str::<ApiKeyStore>(&read_to_string(path)?) {
                    Ok(_) => Ok(()),
                    Err(err) => anyhow::bail!("{}", err),
                }
            }

            ExclusiveConfigArgument::Show => {
                let path = get_configuration_file_path()?;
                let toml_data = toml::from_str::<ApiKeyStore>(
                    read_to_string(path)?.trim(),
                )?;
                toml_data.print_key_value_pairs()?;
                Ok(())
            }
        },
        Err(err) => {
            eprintln!("{}", err);
            Ok(())
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
                format!("IP address at line {} is invalid", index + 1)
            })
        })
        .collect::<Vec<anyhow::Result<IpAddr>>>();

    Ok(ip_addresses)
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
    Ok(results)
}

/// Loads the configuration file for the API tokens for each provider
pub fn load_configuration() -> Result<ApiKeyStore, ConfyError> {
    confy::load::<ApiKeyStore>("geolocate", None)
}

/// Retrieves the path to the default configuration file
pub fn get_configuration_file_path() -> Result<PathBuf, ConfyError> {
    confy::get_configuration_file_path("geolocate", None)
}

/// Opens the configuration file with the users preferred editor.
/// The function will look for the $EDITOR env var and uses `nano`
/// as a fallback. Geolocate will wait for the editor to close before
/// doing anything else.
pub fn open_config_file_with_preferred_editor() -> anyhow::Result<()> {
    let path = get_configuration_file_path()?;
    let editor = std::env::var("EDITOR").unwrap_or("nano".to_string());
    Command::new(editor).arg(&path).spawn()?.wait()?;
    Ok(())
}

/// Convenience function to print the path to the configuration file
pub fn print_configuration_file_path() -> anyhow::Result<()> {
    println!("{}", get_configuration_file_path()?.display());
    Ok(())
}

#[cfg(test)]
mod test {
    use super::read_ip_addresses_from_file;
    use std::path::PathBuf;

    #[test]
    fn ip_addrs_are_read_from_file_correctly() {
        let path = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .parent()
            .unwrap()
            .join("tests")
            .join("ip_addrs.txt");
        let addrs = read_ip_addresses_from_file(path).unwrap();
        assert!(addrs.iter().all(|item| item.is_ok()) == true)
    }

    #[test]
    fn ip_addr_format_is_wrong() {
        let path = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .parent()
            .unwrap()
            .join("tests")
            .join("ip_addrs_with_error.txt");
        let addr = read_ip_addresses_from_file(path).unwrap();
        assert!(addr[1].is_err());
        assert!(
            addr[1].as_ref().unwrap_err().to_string()
                == "IP address at line 2 is invalid"
        )
    }

    #[test]
    fn file_with_ip_addrs_doesnt_exist() {
        let path = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .parent()
            .unwrap()
            .join("tests")
            .join("doesntexist.txt");
        assert!(read_ip_addresses_from_file(path).is_err())
    }
}
