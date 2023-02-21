use colored::Colorize;
use serde::{Deserialize, Serialize};
use std::{
    collections::HashMap,
    io::{stdout, BufWriter, Write},
};

use super::parser::ConfigArguments;

/// Contains the API keys for geolocation data providers.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(transparent)]
pub struct ApiKeyStore(HashMap<String, String>);

impl ApiKeyStore {
    pub fn ip2location_token(&self) -> anyhow::Result<&str> {
        self.0
            .get("ip2location")
            .map(|token| token.as_str())
            .ok_or_else(|| anyhow::anyhow!("Token for provider 'ip2location' is not set. Set it with the config command"))
    }

    pub fn ipgeolocation_token(&self) -> anyhow::Result<&str> {
        self.0.get("ipgeolocation")
        .map(|token| token.as_str())
        .ok_or_else(|| anyhow::anyhow!("Token for provider 'ipgeolocation' is not set. Set it with the config command"))
    }

    pub fn print_key_value_pairs(&self) -> anyhow::Result<()> {
        let mut stdout = BufWriter::new(stdout());
        stdout.write_all(b"\n")?;
        for (key, value) in self.0.clone().drain() {
            stdout.write_fmt(format_args!(
                "{} = {}\n",
                key.bright_cyan().bold(),
                value.green()
            ))?;
        }
        stdout.flush()?;
        Ok(())
    }
}

pub fn operate_on_config_file(
    arguments: ConfigArguments,
    store: ApiKeyStore,
) -> anyhow::Result<()> {
    if arguments.show {
        store.print_key_value_pairs()?;
        Ok(())
    } else if arguments.edit {
        loader::open_configuration_file_with_editor()?;
        Ok(())
    } else if arguments.print_path {
        let path = loader::load_configuration_file_path()?;
        println!("{}", path.display());
        Ok(())
    } else {
        Ok(())
    }
}

pub mod loader {
    use super::ApiKeyStore;
    use anyhow::Context;
    use std::{
        env::VarError,
        path::{Path, PathBuf},
        process::{Command, ExitStatus},
    };

    pub fn load_configuration_file() -> Result<ApiKeyStore, confy::ConfyError> {
        confy::load("geolocate", None)
    }

    fn spawn_editor(editor: &str, path: &Path) -> anyhow::Result<ExitStatus> {
        Command::new(editor)
            .arg(path)
            .spawn()?
            .wait()
            .with_context(|| {
                anyhow::anyhow!(
                    "Opening configuration file at path {} with {} failed",
                    path.display(),
                    editor
                )
            })
    }

    pub fn open_configuration_file_with_editor() -> anyhow::Result<ApiKeyStore>
    {
        let editor = std::env::var("EDITOR")
            .or_else(|_| Ok::<_, VarError>("nano".to_owned()))?;
        let path = load_configuration_file_path()?;
        match load_configuration_file() {
            Ok(key_store) => {
                spawn_editor(&editor, &path)?;
                Ok(key_store)
            }
            Err(err) => match err {
                confy::ConfyError::BadTomlData(_) => {
                    spawn_editor(&editor, &path)?;
                    anyhow::bail!("{}", err)
                }

                err => anyhow::bail!("{}", err),
            },
        }
    }

    pub fn load_configuration_file_path() -> Result<PathBuf, confy::ConfyError>
    {
        confy::get_configuration_file_path("geolocate", None)
    }
}
