use json_color::Colorizer;
use reqwest::{Client, Url};
use std::{net::IpAddr, str::FromStr};

pub trait IpGeolocationState {}
struct BuildState;
struct FetchState;

impl IpGeolocationState for BuildState {}
impl IpGeolocationState for FetchState {}

impl From<IpGeolocation<BuildState>> for IpGeolocation<FetchState> {
    fn from(value: IpGeolocation<BuildState>) -> Self {
        Self {
            url: value.url,
            client: value.client,
            marker: std::marker::PhantomData,
        }
    }
}

#[derive(Clone, Debug)]
pub struct IpGeolocation<State>
where
    State: IpGeolocationState,
{
    url: Url,
    client: Client,
    marker: std::marker::PhantomData<State>,
}

impl IpGeolocation<BuildState> {
    pub fn new() -> Self {
        Self {
            url: Url::from_str("https://api.ipgeolocation.io/ipgeo").unwrap(),
            client: reqwest::Client::new(),
            marker: std::marker::PhantomData,
        }
    }

    pub fn set_api_token(&mut self, api_token: &str) -> &mut Self {
        self.url.set_query(Some(&format!("apiKey={}", api_token)));
        self
    }

    pub fn set_ip_address(&mut self, ip: IpAddr) -> &mut Self {
        self.url
            .query_pairs_mut()
            .append_pair("ip", &ip.to_string());
        self
    }

    pub fn build(self) -> IpGeolocation<FetchState> {
        IpGeolocation::<FetchState>::from(self)
    }
}

impl IpGeolocation<FetchState> {
    pub async fn json(&self) -> anyhow::Result<IpGeolocationResponse> {
        let json = self
            .client
            .get(self.url.clone())
            .send()
            .await?
            .json::<IpGeolocationResponse>()
            .await?;
        Ok(json)
    }
}

impl IpGeolocationResponse {
    pub fn colorize(&self) -> anyhow::Result<String> {
        let colorizer = Colorizer::arbitrary();
        let json_string = serde_json::to_string_pretty(&self.0)?;
        Ok(colorizer.colorize_json_str(&json_string)?)
    }
}

#[derive(serde::Serialize, serde::Deserialize)]
#[serde(transparent)]
pub struct IpGeolocationResponse(serde_json::Map<String, serde_json::Value>);

pub async fn response_from_ipgeolocation(
    arguments: super::parser::IpGeolocationArguments,
    store: &super::config::ApiKeyStore,
) -> anyhow::Result<()> {
    let mut ipgeolocation = IpGeolocation::new();
    ipgeolocation
        .set_ip_address(arguments.addr)
        .set_api_token(store.ipgeolocation_token()?);
    let fetcher = ipgeolocation.build();
    let response = fetcher.json().await?.colorize()?;
    println!("{}", response);
    Ok(())
}
