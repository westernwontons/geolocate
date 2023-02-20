use std::{net::IpAddr, path::PathBuf};

use crate::geolocate_lib::geolocation::Provider;

/// Clap doesn't allow enums to specify as command values unless
/// the enum only has unit variants. MutualExclusivity defines the behaviour of
/// checking for multiple flags passed and returns an error to tell the user
/// that only one of n flags may be used.
pub trait MutualExclusivity {
    type ExclusiveValue;

    fn check_exclusivity(&self) -> anyhow::Result<Self::ExclusiveValue>;
}

/// To send a successful request to any of the two geolocation data providers
/// we need a minimum of one IP address, which is obtainable from the CLI
/// or from a file. The `addrs` method is an `Option<Vec<IpAddr>>`, because
/// `Clap` has a limitation, where you cannot specify an enum as a value
/// to exclusively use from out of two options.
pub trait GeolocationInput {
    fn addrs(&mut self) -> Option<Vec<IpAddr>>;
    fn file(&mut self) -> Option<PathBuf>;
}

/// Multiple providers are supported that have different
/// query strings needed to build a request. The implementors
/// of this trait should return their corresponding providers.
pub trait GeolocationProvider {
    fn provider(&self) -> Provider;
}
