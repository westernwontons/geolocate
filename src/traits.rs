use std::{net::IpAddr, path::PathBuf};

/// Clap doesn't allow enums to specify as command values unless
/// the enum only has unit variants. MutualExclusivity defines the behaviour of
/// checking for multiple flags passed and returns an error to tell the user
/// that only one of n flags may be used.
pub trait MutualExclusivity {
    type ExclusiveValue;

    fn check_exclusivity(&self) -> anyhow::Result<Self::ExclusiveValue>;
}

pub trait GeolocationInput {
    fn addrs(&mut self) -> Option<Vec<IpAddr>>;
    fn file(&mut self) -> Option<PathBuf>;
}
