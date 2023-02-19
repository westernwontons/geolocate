use crate::traits::MutualExclusivity;
use clap::{Args, Parser, Subcommand};
use std::{net::IpAddr, path::PathBuf};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct CommandLineArguments {
    #[command(subcommand)]
    pub commands: Subcommands
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
    Config(ConfigArguments)
}

#[derive(Debug, Args)]
pub struct Ip2LocationArguments {
    /// IP Address to fetch geolocation data about. Can be IPv4 or IPv6
    #[arg(short, long)]
    pub addrs: Option<Vec<IpAddr>>,

    /// File to read IP addresses from
    #[arg(short, long)]
    pub file: Option<PathBuf>
}

/// Arguments to the geolocation data providers are meant to be exclusive.
/// The user can either provide a list of IP addresses or a file containing
/// newline delimited list of IP addresses
pub enum ExclusiveGeolocationArgument {
    IpAddresses,
    File
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

#[derive(Debug, Args)]
pub struct IpGeolocationArguments {
    /// IP Address to fetch geolocation data about. Can be IPv4 or IPv6
    #[arg(short, long)]
    pub addrs: Option<Vec<IpAddr>>,

    /// File to read IP addresses from
    #[arg(short, long)]
    pub file: Option<PathBuf>
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

/// Config arguments are meant to be exclusive.
/// The config file is either printed to the console
/// or it's opened in the users favourite editor
pub enum ExclusiveConfigArgument {
    Show,
    Edit
}

#[derive(Debug, Args)]
pub struct ConfigArguments {
    /// Print the configuration file's contents
    #[arg(short, long)]
    show: bool,

    /// Edit the configuration file
    #[arg(short, long)]
    edit: bool
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
