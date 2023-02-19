use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Contains the API keys for geolocation data providers.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(transparent)]
pub struct ApiKeyStore(HashMap<String, String>);

impl ApiKeyStore {
    /// Gets the API token for ip2location
    pub fn ip2location(&self) -> anyhow::Result<&str> {
        match self.0.get("ip2location") {
            Some(api_key) => Ok(api_key.as_str()),
            None => anyhow::bail!("No token for ip2location specified. Set it with the 'config' subcommand.")
        }
    }

    /// Sets the API token for ip2location
    pub fn set_ip2location_token(
        &mut self,
        token: String
    ) -> anyhow::Result<()> {
        self.0.insert("ip2location".to_string(), token);
        anyhow::Ok(())
    }

    /// Gets the API token for ipgeolocation
    pub fn ipgeolocation(&self) -> anyhow::Result<&str> {
        match self.0.get("ipgeolocation") {
            Some(api_key) => Ok(api_key.as_str()),
            None => anyhow::bail!("No token for ipgeolocation specified. Set it with the 'config' subcommand.")
        }
    }

    /// Sets the API token for ipgeolocation
    pub fn set_ipgeolocation_token(
        &mut self,
        token: String
    ) -> anyhow::Result<()> {
        self.0.insert("ipgeolocation".to_string(), token);
        anyhow::Ok(())
    }
}
