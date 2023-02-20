use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use crate::geolocation::Provider;

/// Contains the API keys for geolocation data providers.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(transparent)]
pub struct ApiKeyStore(HashMap<String, String>);

impl ApiKeyStore {
    pub fn get_provider_token(
        &self,
        provider: &Provider,
    ) -> anyhow::Result<String> {
        match self.0.get(&provider.to_string()) {
            Some(api_key) => Ok(api_key.to_owned()),
            None => anyhow::bail!("No token for ipgeolocation specified. Set it with the 'config' subcommand.")
        }
    }
}
