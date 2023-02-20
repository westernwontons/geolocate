use colored::Colorize;
use serde::{Deserialize, Serialize};
use std::{
    collections::HashMap,
    io::{stdout, BufWriter, Write},
};

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

    pub fn print_key_value_pairs(&self) -> anyhow::Result<()> {
        let mut stdout = BufWriter::new(stdout());
        stdout.write(b"\n")?;
        for (key, value) in self.0.clone().drain() {
            stdout.write_fmt(format_args!(
                "{} = {}\n",
                key.bright_cyan().bold(),
                value.green()
            ))?;
        }
        stdout.flush()?;
        anyhow::Ok(())
    }
}
