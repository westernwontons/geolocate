use clap::{Args, Parser, Subcommand};
use std::{net::IpAddr, path::PathBuf};
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct CommandLineArguments {
    #[command(subcommand)]
    pub command: Command,
}

#[derive(Debug, Subcommand)]
pub enum Command {
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
    pub addr: IpAddr,
}

#[derive(Debug, Args)]
pub struct IpGeolocationArguments {
    /// IP Address to fetch geolocation data about. Can be IPv4 or IPv6
    pub addr: IpAddr,
}

#[derive(Debug, Args)]
pub struct ConfigArguments {
    /// Print the configuration file's contents
    #[arg(short, long)]
    pub show: bool,

    /// Edit the configuration file
    #[arg(short, long, conflicts_with = "show")]
    pub edit: bool,

    /// Print the path to the configuration file
    #[arg(long, conflicts_with_all = &["show", "edit"])]
    pub print_path: bool,
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
