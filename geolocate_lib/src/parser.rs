use crate::{
    geolocation::Provider,
    traits::{GeolocationInput, GeolocationProvider, MutualExclusivity},
};
use clap::{Args, Parser, Subcommand};
use std::{net::IpAddr, path::PathBuf};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct CommandLineArguments {
    #[command(subcommand)]
    pub command: Subcommands,
}

#[derive(Debug, Subcommand)]
pub enum Subcommands {
    /// Use the ip2location API
    #[command(arg_required_else_help = true)]
    Ip2location(Ip2LocationArguments),

    /// Use the ipgeolocation API
    #[command(arg_required_else_help = true)]
    Ipgeolocation(IpGeolocationArguments),

    /// Print the current configuration
    #[command(arg_required_else_help = true)]
    Config(ConfigArguments),

    /// Generate shell completions
    #[command(subcommand)]
    Completions(ShellCompletions),
}

#[derive(Debug, Args)]
pub struct Ip2LocationArguments {
    /// IP Address to fetch geolocation data about. Can be IPv4 or IPv6
    #[arg(short, long, num_args = 1.., value_delimiter = ' ')]
    pub addrs: Option<Vec<IpAddr>>,

    /// File to read IP addresses from
    #[arg(short, long)]
    pub file: Option<PathBuf>,
}

/// Arguments to the geolocation data providers are meant to be exclusive.
/// The user can either provide a list of IP addresses or a file containing
/// newline delimited list of IP addresses
pub enum ExclusiveGeolocationArgument {
    IpAddresses,
    File,
}

impl MutualExclusivity for Ip2LocationArguments {
    type ExclusiveValue = ExclusiveGeolocationArgument;

    fn check_exclusivity(&self) -> anyhow::Result<Self::ExclusiveValue> {
        match (self.addrs.as_deref(), self.file.as_deref()) {
            (Some(_), None) => {
                anyhow::Ok(ExclusiveGeolocationArgument::IpAddresses)
            }
            (None, Some(_)) => anyhow::Ok(ExclusiveGeolocationArgument::File),
            (Some(_), Some(_)) => anyhow::bail!(
                "--file and --addrs arguments are mutually exclusive"
            ),
            (None, None) => {
                anyhow::bail!(
                    "Either --file or --addrs has to be passed, but not both"
                )
            }
        }
    }
}

/// Both addrs or file whenever are used are consumed
/// and there's no need to keep them around.
impl GeolocationInput for Ip2LocationArguments {
    fn addrs(&mut self) -> Option<Vec<IpAddr>> {
        self.addrs.take()
    }

    fn file(&mut self) -> Option<PathBuf> {
        self.file.take()
    }
}

impl GeolocationProvider for Ip2LocationArguments {
    fn provider(&self) -> Provider {
        Provider::Ip2Location
    }
}

#[derive(Debug, Args)]
pub struct IpGeolocationArguments {
    /// IP Address to fetch geolocation data about. Can be IPv4 or IPv6
    #[arg(short, long, num_args = 1.., value_delimiter = ' ')]
    pub addrs: Option<Vec<IpAddr>>,

    /// File to read IP addresses from
    #[arg(short, long)]
    pub file: Option<PathBuf>,
}

impl MutualExclusivity for IpGeolocationArguments {
    type ExclusiveValue = ExclusiveGeolocationArgument;

    fn check_exclusivity(&self) -> anyhow::Result<Self::ExclusiveValue> {
        match (self.addrs.as_deref(), self.file.as_deref()) {
            (Some(_), None) => {
                anyhow::Ok(ExclusiveGeolocationArgument::IpAddresses)
            }
            (None, Some(_)) => anyhow::Ok(ExclusiveGeolocationArgument::File),
            (Some(_), Some(_)) => anyhow::bail!(
                "--file and --addrs arguments are mutually exclusive"
            ),
            (None, None) => {
                anyhow::bail!(
                    "Either --file or --addrs has to be passed, but not both"
                )
            }
        }
    }
}

impl GeolocationInput for IpGeolocationArguments {
    fn addrs(&mut self) -> Option<Vec<IpAddr>> {
        self.addrs.take()
    }

    fn file(&mut self) -> Option<PathBuf> {
        self.file.take()
    }
}

impl GeolocationProvider for IpGeolocationArguments {
    fn provider(&self) -> Provider {
        Provider::IpGeolocation
    }
}

/// Config arguments are meant to be exclusive.
/// The config file is either printed to the console
/// or it's opened in the users favourite editor
pub enum ExclusiveConfigArgument {
    Show,
    Edit,
}

#[derive(Debug, Args)]
pub struct ConfigArguments {
    /// Print the configuration file's contents
    #[arg(short, long)]
    show: bool,

    /// Edit the configuration file
    #[arg(short, long)]
    edit: bool,

    /// Print the path to the configuration file
    #[arg(long)]
    print_path: bool,
}

impl ConfigArguments {
    pub fn print_path(&self) -> anyhow::Result<bool> {
        if self.print_path {
            return anyhow::Ok(true);
        }
        anyhow::Ok(false)
    }
}

impl MutualExclusivity for ConfigArguments {
    type ExclusiveValue = ExclusiveConfigArgument;

    fn check_exclusivity(&self) -> anyhow::Result<Self::ExclusiveValue> {
        match (self.show, self.edit) {
            (false, true) => anyhow::Ok(ExclusiveConfigArgument::Edit),
            (true, false) => anyhow::Ok(ExclusiveConfigArgument::Show),
            (true, true) => anyhow::bail!(
                "Arguments --edit and --show are mutually exclusive"
            ),
            (false, false) => {
                anyhow::bail!("Either --edit or --show has to be provided")
            }
        }
    }
}

#[derive(Debug, Subcommand)]
pub enum ShellCompletions {
    Bash(OutDir),
    Zsh(OutDir),
    Fish(OutDir),
    PowerShell(OutDir),
    Elvish(OutDir),
}

#[derive(Debug, Args)]
pub struct OutDir {
    pub path: Option<PathBuf>,
}
