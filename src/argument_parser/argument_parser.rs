use super::traits::MutualExclusivity;
use clap::{Args, Parser, Subcommand};
use std::{net::IpAddr, path::PathBuf};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct CommandLineArguments {
    #[command(subcommand)]
    commands: Subcommands
}

#[derive(Debug, Subcommand)]
enum Subcommands {
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
    addrs: Option<Vec<IpAddr>>,

    /// File to read IP addresses from
    #[arg(short, long)]
    file: Option<PathBuf>
}

enum ExclusiveArgument {
    IpAddresses,
    File
}

impl MutualExclusivity<ExclusiveArgument> for Ip2LocationArguments {
    fn check_for_exclusivity(&self) -> anyhow::Result<ExclusiveArgument> {
        match (self.addrs.as_deref(), self.file.as_deref()) {
            (Some(_), None) => anyhow::Ok(ExclusiveArgument::IpAddresses),
            (None, Some(_)) => anyhow::Ok(ExclusiveArgument::File),
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
    addrs: Option<Vec<IpAddr>>,

    /// File to read IP addresses from
    #[arg(short, long)]
    file: Option<PathBuf>
}

impl MutualExclusivity<ExclusiveArgument> for IpGeolocationArguments {
    fn check_for_exclusivity(&self) -> anyhow::Result<ExclusiveArgument> {
        match (self.addrs.as_deref(), self.file.as_deref()) {
            (Some(_), None) => anyhow::Ok(ExclusiveArgument::IpAddresses),
            (None, Some(_)) => anyhow::Ok(ExclusiveArgument::File),
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
struct ConfigArguments {
    /// Print the configuration file's contents
    #[arg(short, long)]
    show: bool,

    /// Edit the configuration file
    #[arg(short, long)]
    edit: bool
}
