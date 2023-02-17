use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(transparent)]
pub struct ApiKeysStore(HashMap<String, String>);

impl ApiKeysStore {
    pub fn ip2location(&self) -> anyhow::Result<&str> {
        match self.0.get("ip2location") {
            Some(api_key) => Ok(api_key.as_str()),
            None => anyhow::bail!("No token for ip2location specified. Set it with the 'config' subcommand.")
        }
    }

    pub fn ipgeolocation(&self) -> anyhow::Result<&str> {
        match self.0.get("ipgeolocation") {
            Some(api_key) => Ok(api_key.as_str()),
            None => anyhow::bail!("No token for ipgeolocation specified. Set it with the 'config' subcommand.")
        }
    }
}
