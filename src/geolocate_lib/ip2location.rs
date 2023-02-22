use json_color::Colorizer;
use reqwest::{blocking::Client, Url};
use std::{net::IpAddr, str::FromStr};

pub trait Ip2LocationState {}
pub struct BuildState;
pub struct FetchState;

impl Ip2LocationState for BuildState {}
impl Ip2LocationState for FetchState {}

#[derive(Clone, Debug)]
pub struct Ip2Location<State>
where
    State: Ip2LocationState,
{
    url: Url,
    client: Client,
    marker: std::marker::PhantomData<State>,
}

impl From<Ip2Location<BuildState>> for Ip2Location<FetchState> {
    fn from(value: Ip2Location<BuildState>) -> Self {
        Self {
            url: value.url,
            client: value.client,
            marker: std::marker::PhantomData,
        }
    }
}

impl Ip2Location<BuildState> {
    pub fn new() -> Self {
        Self {
            url: Url::from_str("https://api.ip2location.io").unwrap(),
            client: Client::new(),
            marker: std::marker::PhantomData,
        }
    }
    /// Set the API token
    pub fn set_api_token(&mut self, api_token: &str) -> &mut Self {
        self.url.set_query(Some(&format!("key={}", api_token)));
        self
    }

    /// Set the IP address for the lookup
    pub fn set_ip_address(&mut self, ip_addr: IpAddr) -> &mut Self {
        self.url
            .query_pairs_mut()
            .append_pair("ip", &ip_addr.to_string());
        self
    }

    pub fn build(self) -> Ip2Location<FetchState> {
        Ip2Location::<FetchState>::from(self)
    }
}

impl Ip2Location<FetchState> {
    /// Get the geolocation data of an IP address from
    /// the `ip2location` provider
    pub fn json(&self) -> anyhow::Result<Ip2LocationResponse> {
        let json = self
            .client
            .get(self.url.as_str())
            .send()?
            .json::<Ip2LocationResponse>()?;
        Ok(json)
    }
}

#[derive(serde::Serialize, serde::Deserialize)]
#[serde(transparent)]
pub struct Ip2LocationResponse(serde_json::Map<String, serde_json::Value>);

impl Ip2LocationResponse {
    /// Colorize the JSON response of `ip2location`
    pub fn colorize(&self) -> anyhow::Result<String> {
        let colorizer = Colorizer::arbitrary();
        let json_string = serde_json::to_string_pretty(&self.0)?;
        Ok(colorizer.colorize_json_str(&json_string)?)
    }
}

pub fn response_from_ip2location(
    arguments: super::parser::Ip2LocationArguments,
    store: &super::config::ApiKeyStore,
) -> anyhow::Result<()> {
    let mut ip2location = Ip2Location::new();
    ip2location
        .set_ip_address(arguments.addr)
        .set_api_token(store.ip2location_token()?);
    let fetcher = ip2location.build();
    let response = fetcher.json()?.colorize()?;
    println!("{}", response);
    Ok(())
}
