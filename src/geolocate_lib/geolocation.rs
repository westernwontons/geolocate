use crate::geolocate_lib::helpers::fetch_many;
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use std::{fmt::Display, net::IpAddr};

/// The supported geolocation data providers.
/// Their API will be used to fetch geo data from.
#[derive(Clone, Debug, Eq, Hash, PartialEq, Serialize, Deserialize)]
pub enum Provider {
    Ip2Location,
    IpGeolocation,
}

impl Display for Provider {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Provider::Ip2Location => write!(f, "ip2location"),
            Provider::IpGeolocation => write!(f, "ipgeolocation"),
        }
    }
}

/// Represents the operation of geolocation.
/// Give it IP address/addresses and an API key
/// and will give you geolocation data.
#[derive(Clone, Debug)]
pub struct Geolocation {
    ip_addrs: Vec<IpAddr>,
    api_key: String,
    client: reqwest::Client,
}

impl Geolocation {
    pub fn new(ip_addrs: Vec<IpAddr>, api_key: String) -> Self {
        Self {
            ip_addrs,
            api_key,
            client: reqwest::Client::new(),
        }
    }

    /// Fetch geolocation data from a provider.
    pub async fn fetch<T>(
        &mut self,
        provider: Provider,
    ) -> anyhow::Result<Vec<T>>
    where
        T: Serialize + DeserializeOwned + Send + 'static,
    {
        match provider {
            Provider::Ip2Location => match self.ip_addrs.len() {
                1 => {
                    let url = format!(
                        "https://api.ip2location.io/?ip={}&key={}",
                        self.ip_addrs.first().unwrap(),
                        self.api_key
                    );
                    let json_result = self
                        .client
                        .get(url)
                        .send()
                        .await?
                        .json::<T>()
                        .await
                        .map_err(|err| anyhow::anyhow!("{}", err));
                    match json_result {
                        Ok(json) => Ok(vec![json]),
                        Err(err) => anyhow::bail!("{}", err),
                    }
                }
                _ => self.fetch_many(provider).await,
            },
            Provider::IpGeolocation => match self.ip_addrs.len() {
                1 => {
                    let url = format!(
                        "https://api.ipgeolocation.io/ipgeo?apiKey={}&ip={}",
                        self.api_key,
                        self.ip_addrs.first().unwrap()
                    );
                    let json_result = self
                        .client
                        .get(url)
                        .send()
                        .await?
                        .json::<T>()
                        .await
                        .map_err(|err| anyhow::anyhow!("{}", err));
                    match json_result {
                        Ok(json) => Ok(vec![json]),
                        Err(err) => anyhow::bail!("{}", err),
                    }
                }
                _ => self.fetch_many(provider).await,
            },
        }
    }

    /// Fetch geolocation data from a provider,
    /// but for multiple IP addresses.
    /// Initiates concurrent requests.
    async fn fetch_many<T>(
        &mut self,
        provider: Provider,
    ) -> anyhow::Result<Vec<T>>
    where
        T: Serialize + DeserializeOwned + Send + 'static,
    {
        match provider {
            Provider::Ip2Location => {
                let urls = self
                    .ip_addrs
                    .iter()
                    .map(|ip_addr| {
                        format!(
                            "https://api.ip2location.io/?ip={}&key={}",
                            ip_addr, self.api_key
                        )
                    })
                    .collect::<Vec<String>>();
                fetch_many(urls, &self.client).await
            }

            Provider::IpGeolocation => {
                let urls = self.ip_addrs
                    .iter()
                    .map(|ip_addr| {
                        format!(
                            "https://api.ipgeolocation.io/ipgeo?apiKey={}&ip={}",
                            self.api_key, ip_addr
                        )
                    })
                    .collect::<Vec<String>>();
                fetch_many(urls, &self.client).await
            }
        }
    }
}
