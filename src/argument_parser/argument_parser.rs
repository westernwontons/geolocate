#![allow(
    unused_variables,
    unused_mut,
    unused_imports,
    dead_code,
    unused_assignments
)]

use clap::{Args, CommandFactory, Parser, Subcommand};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct CommandLineArguments {
    #[command(subcommand)]
    command: Subcommands
}

#[derive(Debug, Subcommand)]
#[command(name = "ip2location")]
enum Subcommands {
    Ip2location(Ip2LocationArgs),
    Ipgeolocation(IpGeolocationArgs)
}

#[derive(Debug, Args)]
struct Ip2LocationArgs {}

#[derive(Debug, Args)]
struct IpGeolocationArgs {}
