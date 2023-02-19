#![allow(
    unused_variables,
    unused_mut,
    unused_imports,
    dead_code,
    unused_assignments
)]

use crate::config::ApiKeyStore;
use serde::{Deserialize, Serialize};
use std::{fs::read_to_string, net::IpAddr, path::PathBuf};

#[derive(Clone, Debug)]
pub enum Provider {
    Ip2Location,
    IpGeolocation
}

#[derive(thiserror::Error, Debug)]
pub enum GeolocationBuildError {
    #[error(transparent)]
    ConfigurationLoadError(#[from] confy::ConfyError),
    #[error("API token is missing")]
    TokenMissingError,
    #[error("Must provide at least one IP address")]
    NoIpAddressProvidedError,
    #[error("File is not good")]
    FileNotGood(#[from] std::io::Error),
    #[error("Invalid IP address in file")]
    InvalidIpAddress
}

#[derive(Clone, Debug)]
pub struct Geolocation {
    ip_addresses: Vec<IpAddr>,
    api_key: String,
    client: reqwest::Client,
    provider: Provider
}

impl Geolocation {
    pub fn try_new(
        ip_addresses: Vec<IpAddr>,
        provider: Provider,
        store: ApiKeyStore,
        client: reqwest::Client
    ) -> Result<Self, GeolocationBuildError> {
        let api_key = match provider {
            Provider::Ip2Location => store.ip2location(),
            Provider::IpGeolocation => store.ipgeolocation()
        };

        let api_key = match api_key {
            Ok(key) => key.to_owned(),
            Err(err) => return Err(GeolocationBuildError::TokenMissingError)
        };

        if ip_addresses.len() < 1 {
            return Err(GeolocationBuildError::NoIpAddressProvidedError);
        }

        Ok(Self {
            ip_addresses,
            api_key,
            client,
            provider
        })
    }

    pub fn try_new_from_file(
        file: PathBuf,
        provider: Provider,
        store: ApiKeyStore,
        client: reqwest::Client
    ) -> Result<Self, GeolocationBuildError> {
        let content = read_to_string(file)?;
        let mut ip_addresses = content
            .split_terminator("\n")
            .into_iter()
            .map(|item| item.parse::<IpAddr>());
        if ip_addresses.by_ref().any(|item| item.is_err()) {
            return Err(GeolocationBuildError::InvalidIpAddress);
        }

        Geolocation::try_new(
            ip_addresses.into_iter().map(|item| item.unwrap()).collect(),
            Provider::Ip2Location,
            store,
            client
        )
    }

    /// Fetch the geolocation data from a provider
    pub async fn fetch(
        &self
    ) -> Result<serde_json::Map<String, serde_json::Value>, reqwest::Error>
    {
        match self.provider {
            Provider::Ip2Location => {
                self.client
                    .get(format!(
                        "https://api.ip2location.io/?ip={}&key={}",
                        self.ip_addresses.first().unwrap(),
                        self.api_key
                    ))
                    .send()
                    .await?
                    .json::<serde_json::Map<String, serde_json::Value>>()
                    .await
            }
            Provider::IpGeolocation => {
                self.client
                    .get(format!(
                        "https://api.ipgeolocation.io/ipgeo?apiKey={}&ip={}",
                        self.api_key,
                        self.ip_addresses.first().unwrap()
                    ))
                    .send()
                    .await?
                    .json()
                    .await
            }
        }
    }

    pub fn fetch_many() -> Vec<Result<serde_json::Value, reqwest::Error>> {
        todo!()
    }
}
